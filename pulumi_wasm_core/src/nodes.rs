use crate::model::MaybeNodeValue::{NotYetCalculated, Set};
use crate::model::NodeValue::Nothing;
use crate::model::{FieldName, FunctionName, MaybeNodeValue, NodeValue, OutputId};
use crate::pulumi::service::{PerformResourceRequest, RegisterResourceResponse};
use log::error;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Callback {
    CreateResource(OutputId, FieldName),
    ExtractField(OutputId),
    NativeFunction(OutputId),
    CombineOutputs(OutputId, u32),
}

impl Callback {
    pub(crate) fn create_resource(output_id: OutputId, field_name: FieldName) -> Self {
        Self::CreateResource(output_id, field_name)
    }

    pub(crate) fn extract_field(output_id: OutputId) -> Self {
        Self::ExtractField(output_id)
    }

    pub(crate) fn native_function(output_id: OutputId) -> Self {
        Self::NativeFunction(output_id)
    }
    pub(crate) fn combine_outputs(output_id: OutputId, count: u32) -> Self {
        Self::CombineOutputs(output_id, count)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct DoneNode {
    value: NodeValue, // In reality Done have only Value, but being able to set Nothing is useful for testing
    callbacks: Vec<Callback>,
}

impl DoneNode {
    pub(crate) fn create(value: Value, callbacks: Vec<Callback>) -> Self {
        Self {
            value: value.into(),
            callbacks,
        }
    }
    pub(crate) fn new(value: Value) -> Self {
        DoneNode::create(value, Vec::new())
    }

    pub(crate) fn get_value(&self) -> &NodeValue {
        &self.value
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct NativeFunctionNode {
    argument: MaybeNodeValue,
    value: MaybeNodeValue,
    function_name: FunctionName,
    callbacks: Vec<Callback>,
}

impl NativeFunctionNode {
    pub(crate) fn new(function_name: FunctionName) -> Self {
        Self {
            argument: NotYetCalculated,
            value: NotYetCalculated,
            function_name,
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn get_argument_value(&self) -> &Value {
        match &self.argument {
            MaybeNodeValue::NotYetCalculated => {
                error!("Argument is not yet calculated");
                panic!("Argument is not yet calculated");
            }
            Set(NodeValue::Nothing) => {
                error!("Argument is Nothing");
                panic!("Argument is Nothing");
            }
            Set(NodeValue::Exists(value)) => value,
        }
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    pub(crate) fn get_function_name(&self) -> &FunctionName {
        &self.function_name
    }

    pub(crate) fn set_argument(&mut self, value: NodeValue) {
        self.argument = MaybeNodeValue::Set(value);
    }

    pub(crate) fn set_value(&mut self, value: NodeValue) {
        self.value = MaybeNodeValue::Set(value);
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterResourceRequestOperation {
    pub(crate) name: String,
    pub(crate) r#type: String,
}

impl RegisterResourceRequestOperation {
    pub(crate) fn new(r#type: String, name: String) -> Self {
        Self { name, r#type }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ResourceInvokeRequestOperation {
    pub(crate) token: String,
}
impl ResourceInvokeRequestOperation {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ResourceRequestOperation {
    Register(RegisterResourceRequestOperation),
    Invoke(ResourceInvokeRequestOperation),
}

#[derive(Debug, PartialEq)]
pub(crate) struct AbstractResourceNode {
    value: MaybeNodeValue,
    required_inputs: HashSet<FieldName>,
    inputs: HashMap<FieldName, NodeValue>,
    outputs: HashSet<FieldName>,
    callbacks: Vec<Callback>,
    operation: ResourceRequestOperation,
}

impl AbstractResourceNode {
    pub(crate) fn create(
        value: MaybeNodeValue,
        operation: ResourceRequestOperation,
        required_inputs: HashSet<FieldName>,
        inputs: HashMap<FieldName, NodeValue>,
        outputs: HashSet<FieldName>,
        callbacks: Vec<Callback>,
    ) -> Self {
        Self {
            value,
            operation,
            required_inputs,
            inputs,
            outputs,
            callbacks,
        }
    }

    pub(crate) fn new(
        operation: ResourceRequestOperation,
        input_names: HashSet<FieldName>,
        outputs: HashSet<FieldName>,
    ) -> Self {
        Self::create(
            NotYetCalculated,
            operation,
            input_names,
            HashMap::new(),
            outputs,
            Vec::new(),
        )
    }

    pub(crate) fn set_input(
        &mut self,
        name: FieldName,
        value: NodeValue,
    ) -> Option<PerformResourceRequest> {
        if !self.required_inputs.contains(&name) {
            panic!("Input not found: {:?}", name);
        }
        self.required_inputs.remove(&name);
        self.inputs.insert(name, value);

        if self.required_inputs.is_empty() {
            Some(self.generate_request())
        } else {
            None
        }
    }

    //TODO: Write tests
    pub(crate) fn set_value(&mut self, value: &RegisterResourceResponse) -> NodeValue {
        let map: HashMap<String, Value> = value
            .outputs
            .iter()
            .map(|(k, v)| (k.as_string().clone(), v.clone()))
            .collect();
        let val = Value::Object(map.into_iter().collect());
        let node_value = NodeValue::Exists(val);

        self.value = Set(node_value.clone());
        node_value
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    fn generate_request(&self) -> PerformResourceRequest {
        let mut object = HashMap::new();

        for (name, value) in self.inputs.iter() {
            match value {
                NodeValue::Nothing => {
                    object.insert(name.clone(), None);
                }
                NodeValue::Exists(v) => {
                    object.insert(name.clone(), Some(v.clone()));
                }
            };
        }

        PerformResourceRequest {
            operation: self.operation.clone(),
            object,
            expected_results: self.outputs.clone(),
        }
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ExtractFieldNode {
    value: MaybeNodeValue,
    field_name: FieldName,
    callbacks: Vec<Callback>,
}

impl ExtractFieldNode {
    pub(crate) fn create(
        value: MaybeNodeValue,
        field_name: FieldName,
        callbacks: Vec<Callback>,
    ) -> Self {
        Self {
            value,
            field_name,
            callbacks,
        }
    }

    pub(crate) fn new(field_name: FieldName) -> ExtractFieldNode {
        Self::create(NotYetCalculated, field_name, Vec::new())
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }

    // TODO: Write tests
    pub(crate) fn extract_field(&mut self, node_value: &NodeValue) -> NodeValue {
        match node_value {
            NodeValue::Nothing => {
                error!("Cannot extract field from Nothing");
                panic!("Cannot extract field from Nothing");
            }

            NodeValue::Exists(Value::Object(map)) => {
                let key: Value = self.field_name.as_string().clone().into();
                let value = map.iter().find(|(k, _)| *k == &key).map(|(_, v)| v.clone());
                let new_node_value = match value {
                    None => NodeValue::Nothing,
                    Some(v) => NodeValue::Exists(v),
                };
                self.value = Set(new_node_value.clone());
                new_node_value
            }
            NodeValue::Exists(_) => {
                error!("Cannot extract field from non-Map");
                panic!("Cannot extract field from non-Map");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CombineOutputsNode {
    value: MaybeNodeValue,
    inputs: Vec<Option<Value>>,
    inputs_set: u32,
    callbacks: Vec<Callback>,
}

impl CombineOutputsNode {
    pub(crate) fn new(number_of_inputs: u32) -> Self {
        Self {
            value: NotYetCalculated,
            inputs: vec![None; number_of_inputs as usize],
            inputs_set: 0,
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn set_node_value(&mut self, index: u32, value: NodeValue) -> Option<NodeValue> {
        self.inputs[index as usize] = match value {
            Nothing => None,
            NodeValue::Exists(v) => Some(v),
        };
        self.inputs_set += 1;
        if self.inputs_set == self.inputs.len() as u32 {
            let set_inputs: Vec<_> = self.inputs.iter().filter_map(|v| v.clone()).collect();
            let value: NodeValue = if set_inputs.len() != self.inputs.len() {
                Nothing
            } else {
                let list: Value = set_inputs.into();
                list.clone().into()
            };
            self.value = Set(value.clone());
            Some(value)
        } else {
            None
        }
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::model::NodeValue::{Exists, Nothing};
    use crate::nodes::AbstractResourceNode;
    use serde_json::Value::Null;

    mod register_resource_node {
        use super::*;
        use crate::nodes::ResourceRequestOperation::Register;
        use crate::nodes::{RegisterResourceRequestOperation, ResourceRequestOperation};
        use crate::pulumi::service::PerformResourceRequest;
        use std::collections::HashSet;

        #[test]
        fn set_input_passes_it_to_pulumi() {
            let mut node = AbstractResourceNode::new(
                Register(RegisterResourceRequestOperation::new(
                    "type".into(),
                    "name".into(),
                )),
                ["exists_nil".into(), "exists_int".into(), "not_exist".into()].into(),
                HashSet::from(["output".into()]),
            );

            let result = node.set_input("exists_nil".into(), Exists(Null));
            assert_eq!(result, None);

            let result = node.set_input("exists_int".into(), Exists(2.into()));
            assert_eq!(result, None);

            let result = node.set_input("not_exist".into(), Nothing);
            assert_eq!(
                result,
                Some(PerformResourceRequest {
                    object: HashMap::from([
                        ("exists_nil".into(), Some(Null)),
                        ("exists_int".into(), Some(2.into())),
                        ("not_exist".into(), None),
                    ]),
                    operation: ResourceRequestOperation::Register(
                        RegisterResourceRequestOperation::new("type".into(), "name".into())
                    ),
                    expected_results: HashSet::from(["output".into()]),
                })
            );
        }
    }

    mod combine_outputs_node {
        use super::*;
        use crate::model::MaybeNodeValue::{NotYetCalculated, Set};
        use crate::nodes::CombineOutputsNode;
        use serde_json::json;

        #[test]
        fn set_inputs() {
            let mut node = CombineOutputsNode::new(2);

            let result = node.set_node_value(0, Exists(0.into()));
            assert_eq!(result, None);
            assert_eq!(node.value, NotYetCalculated);

            let result = node.set_node_value(1, Exists("123".into()));
            assert_eq!(result, Some(json!([0, "123"]).into()));
            assert_eq!(node.value, json!([0, "123"]).into());
        }

        #[test]
        fn set_unknown_inputs() {
            let mut node = CombineOutputsNode::new(2);

            let result = node.set_node_value(0, Exists(0.into()));
            assert_eq!(result, None);
            assert_eq!(node.get_value(), &NotYetCalculated);

            let result = node.set_node_value(1, Nothing);
            assert_eq!(result, Some(Nothing));
            assert_eq!(node.get_value(), &Set(Nothing));
        }
    }
}
