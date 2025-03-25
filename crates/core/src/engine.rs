use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Deref;

use crate::config::{Config, RawConfigValue};
use crate::model::NodeValue::Exists;
use crate::model::{FieldName, FunctionName, MaybeNodeValue, NodeValue, OutputId};
use crate::nodes::{
    AbstractResourceNode, Callback, CombineOutputsNode, DoneNode, ExtractFieldNode,
    NativeFunctionNode, RegisterResourceRequestOperation, ResourceInvokeRequestOperation,
    ResourceRequestOperation,
};
use crate::pulumi::service::PulumiService;
use log::error;
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct ForeignFunctionToInvoke {
    pub output_id: OutputId,
    pub function_name: FunctionName,
    pub value: Value,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigValue {
    PlainText(String),
    Secret(OutputId),
}

enum EngineNode {
    Done(DoneNode),
    NativeFunction(NativeFunctionNode),
    RegisterResource(AbstractResourceNode),
    ExtractField(ExtractFieldNode),
    CombineOutputs(CombineOutputsNode),
}

pub(crate) struct Holder<A: ?Sized>(RefCell<A>);

impl<A> From<A> for Holder<A> {
    fn from(value: A) -> Self {
        Holder(RefCell::new(value))
    }
}

impl<A: ?Sized> Holder<A> {
    fn map<U, F>(&self, f: F) -> Ref<U>
    where
        F: FnOnce(&A) -> &U,
    {
        Ref::map(self.get(), f)
    }

    fn map_mut<U, F>(&self, f: F) -> RefMut<U>
    where
        F: FnOnce(&mut A) -> &mut U,
    {
        RefMut::map(self.get_mut(), f)
    }

    fn foreach_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut A),
    {
        RefMut::map(self.get_mut(), |x| {
            f(x);
            x
        });
    }

    fn get(&self) -> Ref<A> {
        self.0.borrow()
    }

    fn get_mut(&self) -> RefMut<A> {
        self.0.borrow_mut()
    }
}

type NodesMap = HashMap<OutputId, Holder<EngineNode>>;

struct EngineView<'a> {
    ready_foreign_function_ids: &'a mut HashSet<OutputId>,
    register_resource_ids: &'a mut HashSet<OutputId>,
    pulumi: &'a dyn PulumiService,
}

pub struct Engine {
    done_node_ids: VecDeque<OutputId>,
    ready_foreign_function_ids: HashSet<OutputId>,
    register_resource_ids: HashSet<OutputId>,

    outputs: HashMap<FieldName, OutputId>,

    pulumi: Box<dyn PulumiService>,
    nodes: NodesMap,
    config: Config,
}

impl Engine {
    pub fn new(pulumi: impl PulumiService + 'static, config: Config) -> Self {
        Self {
            done_node_ids: VecDeque::new(),
            ready_foreign_function_ids: HashSet::new(),
            register_resource_ids: HashSet::new(),
            outputs: HashMap::new(),
            nodes: HashMap::new(),
            pulumi: Box::new(pulumi),
            config,
        }
    }

    #[cfg(test)]
    pub fn new_without_configs(pulumi: impl PulumiService + 'static) -> Self {
        let config = Config::new(HashMap::new(), HashSet::new());
        Self::new(pulumi, config)
    }

    pub fn run(
        &mut self,
        native_function_results: HashMap<OutputId, Value>,
    ) -> Option<Vec<ForeignFunctionToInvoke>> {
        self.loop_over_dones();
        self.loop_over_native_function_results(native_function_results);

        loop {
            if !self.ready_foreign_function_ids.is_empty() {
                return Some(self.prepare_foreign_function_results());
            }

            if self.register_resource_ids.is_empty() {
                break;
            }
            self.handle_creating_resources();
        }

        if !self.outputs.is_empty() {
            let mut value = HashMap::new();

            for (field_name, output_id) in self.outputs.iter() {
                if let Some(output_id) = self.get_value(*output_id) {
                    value.insert(field_name.clone(), output_id);
                }
            }

            self.pulumi.register_outputs(value); //FIXME: Secrets
        }

        None
    }

    pub fn add_output(&mut self, field_name: FieldName, output_id: OutputId) {
        self.outputs.insert(field_name, output_id);
    }

    fn get_value(&self, output_id: OutputId) -> Option<Value> {
        match self.nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => {
                let ndv = match r.0.borrow().deref() {
                    EngineNode::Done(node) => MaybeNodeValue::Set(node.get_value().clone()),
                    EngineNode::NativeFunction(node) => node.get_value().clone(),
                    EngineNode::RegisterResource(node) => node.get_value().clone(),
                    EngineNode::ExtractField(node) => node.get_value().clone(),
                    EngineNode::CombineOutputs(node) => node.get_value().clone(),
                };

                match ndv {
                    MaybeNodeValue::NotYetCalculated => None,
                    MaybeNodeValue::Set(NodeValue::Nothing) => None,
                    MaybeNodeValue::Set(NodeValue::Exists {
                        value: v,
                        secret: false,
                    }) => Some(v),
                    MaybeNodeValue::Set(NodeValue::Exists {
                        value: v,
                        secret: true,
                    }) => Some(json!({
                        crate::constants::SPECIAL_SIG_KEY: crate::constants::SPECIAL_SECRET_SIG,
                        crate::constants::SECRET_VALUE_NAME: v
                    })),
                }
            }
        }
    }

    fn prepare_foreign_function_results(&self) -> Vec<ForeignFunctionToInvoke> {
        self.ready_foreign_function_ids
            .iter()
            .map(|output_id| {
                let node = self.get_native_function(*output_id);
                ForeignFunctionToInvoke {
                    output_id: *output_id,
                    function_name: node.get_function_name().clone(),
                    value: node.get_argument_value().clone(),
                }
            })
            .collect()
    }

    fn loop_over_dones(&mut self) {
        loop {
            let output_id = match self.done_node_ids.pop_back() {
                None => break,
                Some(output_id) => output_id,
            };

            let node = Self::get_done_free(&self.nodes, output_id);

            let value = node.get_value();
            let callbacks = node.get_callbacks();
            let mut sets = EngineView {
                ready_foreign_function_ids: &mut self.ready_foreign_function_ids,
                register_resource_ids: &mut self.register_resource_ids,
                pulumi: self.pulumi.deref(),
            };

            for callback in callbacks {
                Engine::handle_callback(value, callback, &self.nodes, &mut sets);
            }
        }
    }

    fn loop_over_native_function_results(
        &mut self,
        native_function_results: HashMap<OutputId, Value>,
    ) {
        let nodes = &self.nodes;

        for (output_id, value) in native_function_results {
            let mut node = Self::get_native_function_free_mut(nodes, output_id);
            let value = node.set_value(value);
            self.ready_foreign_function_ids.remove(&output_id);

            let mut sets = EngineView {
                ready_foreign_function_ids: &mut self.ready_foreign_function_ids,
                register_resource_ids: &mut self.register_resource_ids,
                pulumi: self.pulumi.deref(),
            };

            Self::run_callbacks(node.get_callbacks(), &value, nodes, &mut sets);
        }
    }

    fn handle_creating_resources(&mut self) {
        let output = self
            .pulumi
            .poll_resource_operations(&self.register_resource_ids);

        for (output_id, response) in output {
            self.register_resource_ids.remove(&output_id);
            let mut node = Self::get_create_resource_free_mut(&self.nodes, output_id);
            let value = node.set_value(&response);
            let mut sets = EngineView {
                ready_foreign_function_ids: &mut self.ready_foreign_function_ids,
                register_resource_ids: &mut self.register_resource_ids,
                pulumi: self.pulumi.deref(),
            };
            Self::run_callbacks(node.get_callbacks(), &value, &self.nodes, &mut sets);
        }
    }

    fn run_callbacks(
        callbacks: &[Callback],
        node_value: &NodeValue,
        nodes: &NodesMap,
        sets: &mut EngineView,
    ) {
        callbacks
            .iter()
            .for_each(|callback| Self::handle_callback(node_value, callback, nodes, sets));
    }

    fn handle_callback(
        value: &NodeValue,
        callback: &Callback,
        nodes: &NodesMap,
        sets: &mut EngineView,
    ) {
        match callback {
            Callback::CreateResource(output_id, field_name) => {
                Self::handle_create_resource_callback(value, nodes, output_id, field_name, sets)
            }
            Callback::ExtractField(output_id) => {
                Self::handle_extract_field_callback(value, nodes, sets, output_id);
            }
            Callback::NativeFunction(output_id) => {
                Self::handle_native_function_callback(value, nodes, sets, output_id);
            }
            Callback::CombineOutputs(output_id, idx) => {
                Self::handle_combine_outputs_callback(value, nodes, sets, output_id, *idx);
            }
        }
    }

    fn handle_create_resource_callback(
        value: &NodeValue,
        nodes: &NodesMap,
        output_id: &OutputId,
        field_name: &FieldName,
        engine_view: &mut EngineView,
    ) {
        let mut create_resource_node = Self::get_create_resource_free_mut(nodes, *output_id);

        let registration_request =
            create_resource_node.set_input(field_name.clone(), value.clone());

        match registration_request {
            None => {}
            Some(rr) => {
                engine_view
                    .pulumi
                    .perform_resource_operation(*output_id, rr);
                engine_view.register_resource_ids.insert(*output_id);
            }
        }
    }

    fn handle_native_function_callback(
        value: &NodeValue,
        nodes: &NodesMap,
        sets: &mut EngineView,
        output_id: &OutputId,
    ) {
        let mut node = Self::get_native_function_free_mut(nodes, *output_id);
        node.set_argument(value.clone());
        match value {
            NodeValue::Nothing => {
                node.set_nothing();
                Self::run_callbacks(node.get_callbacks(), value, nodes, sets);
            }
            Exists {
                value: _,
                secret: _,
            } => {
                sets.ready_foreign_function_ids.insert(*output_id);
            }
        }
    }

    fn handle_combine_outputs_callback(
        value: &NodeValue,
        nodes: &NodesMap,
        sets: &mut EngineView,
        output_id: &OutputId,
        idx: u32,
    ) {
        let mut node = Self::get_combine_outputs_free_mut(nodes, *output_id);
        let result = node.set_node_value(idx, value.clone());
        match result {
            None => {}
            Some(v) => {
                Self::run_callbacks(node.get_callbacks(), &v, nodes, sets);
            }
        }
    }

    fn handle_extract_field_callback(
        value: &NodeValue,
        nodes: &NodesMap,
        mutable_sets: &mut EngineView,
        output_id: &OutputId,
    ) {
        let mut node = Self::get_extract_field_free_mut(nodes, *output_id);
        let new_value = node.extract_field(value);
        Self::run_callbacks(node.get_callbacks(), &new_value, nodes, mutable_sets)
    }

    #[cfg(test)]
    fn get_done(&self, output_id: OutputId) -> Ref<DoneNode> {
        Self::get_done_free(&self.nodes, output_id)
    }

    fn get_done_free(nodes: &NodesMap, output_id: OutputId) -> Ref<DoneNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map(|t| match t {
                EngineNode::Done(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!("Node with id [{}] is native function, not done", output_id);
                    panic!("Node with id [{}] is native function, not done", output_id)
                }
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not done",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not done",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!("Node with id [{}] is extract field, not done", output_id);
                    panic!("Node with id [{}] is extract field, not done", output_id)
                }
                EngineNode::CombineOutputs(_) => {
                    error!("Node with id [{}] is combine outputs, not done", output_id);
                    panic!("Node with id [{}] is combine outputs, not done", output_id)
                }
            }),
        }
    }

    fn get_native_function(&self, output_id: OutputId) -> Ref<NativeFunctionNode> {
        Self::get_native_function_free(&self.nodes, output_id)
    }

    fn get_native_function_free(nodes: &NodesMap, output_id: OutputId) -> Ref<NativeFunctionNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map(|t| match t {
                EngineNode::NativeFunction(node) => node,
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not native function", output_id);
                    panic!("Node with id [{}] is done, not native function", output_id)
                }
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    )
                }
                EngineNode::CombineOutputs(_) => {
                    error!(
                        "Node with id [{}] is combine outputs, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is combine outputs, not native function",
                        output_id
                    )
                }
            }),
        }
    }

    fn get_native_function_free_mut(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> RefMut<NativeFunctionNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map_mut(|t| match t {
                EngineNode::NativeFunction(node) => node,
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not native function", output_id);
                    panic!("Node with id [{}] is done, not native function", output_id)
                }
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    )
                }
                EngineNode::CombineOutputs(_) => {
                    error!(
                        "Node with id [{}] is combine outputs, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is combine outputs, not native function",
                        output_id
                    )
                }
            }),
        }
    }

    #[cfg(test)]
    fn get_extract_field_mut(&self, output_id: OutputId) -> RefMut<ExtractFieldNode> {
        Self::get_extract_field_free_mut(&self.nodes, output_id)
    }

    fn get_extract_field_free_mut(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> RefMut<ExtractFieldNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map_mut(|t| match t {
                EngineNode::ExtractField(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!(
                        "Node with id [{}] is native function, not extract field",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is native function, not extract field",
                        output_id
                    )
                }
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not extract field", output_id);
                    panic!("Node with id [{}] is done, not extract field", output_id)
                }
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not extract field",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not extract field",
                        output_id
                    )
                }
                EngineNode::CombineOutputs(_) => {
                    error!(
                        "Node with id [{}] is combine outputs, not extract field",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is combine outputs, not extract field",
                        output_id
                    )
                }
            }),
        }
    }

    fn get_combine_outputs_free_mut(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> RefMut<CombineOutputsNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map_mut(|t| match t {
                EngineNode::CombineOutputs(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!(
                        "Node with id [{}] is native function, not combine outputs",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is native function, not combine outputs",
                        output_id
                    )
                }
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not combine outputs", output_id);
                    panic!("Node with id [{}] is done, not combine outputs", output_id)
                }
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not combine outputs",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not combine outputs",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not combine outputs",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not combine outputs",
                        output_id
                    )
                }
            }),
        }
    }

    #[cfg(test)]
    fn get_create_resource(&self, output_id: OutputId) -> Ref<AbstractResourceNode> {
        Self::get_create_resource_free(&self.nodes, output_id)
    }

    #[cfg(test)]
    fn get_create_resource_free(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> Ref<AbstractResourceNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map(|t| match t {
                EngineNode::RegisterResource(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!(
                        "Node with id [{}] is native function, not create resource",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is native function, not create resource",
                        output_id
                    )
                }
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not create resource", output_id);
                    panic!("Node with id [{}] is done, not create resource", output_id)
                }

                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not create resource",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not create resource",
                        output_id
                    )
                }
                EngineNode::CombineOutputs(_) => {
                    error!(
                        "Node with id [{}] is combine outputs, not create resource",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is combine outputs, not create resource",
                        output_id
                    )
                }
            }),
        }
    }

    fn get_create_resource_free_mut(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> RefMut<AbstractResourceNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map_mut(|t| match t {
                EngineNode::RegisterResource(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!(
                        "Node with id [{}] is native function, not create resource",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is native function, not create resource",
                        output_id
                    )
                }
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not create resource", output_id);
                    panic!("Node with id [{}] is done, not create resource", output_id)
                }

                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not create resource",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not create resource",
                        output_id
                    )
                }
                EngineNode::CombineOutputs(_) => {
                    error!(
                        "Node with id [{}] is combine outputs, not create resource",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is combine outputs, not create resource",
                        output_id
                    )
                }
            }),
        }
    }

    fn add_callback(&self, output_id: OutputId, callback: Callback) {
        match self.nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => {
                r.foreach_mut(|t| {
                    match t {
                        EngineNode::Done(node) => node.add_callback(callback),
                        EngineNode::NativeFunction(node) => node.add_callback(callback),
                        EngineNode::RegisterResource(node) => node.add_callback(callback),
                        EngineNode::ExtractField(node) => node.add_callback(callback),
                        EngineNode::CombineOutputs(node) => node.add_callback(callback),
                    };
                });
            }
        }
    }

    pub fn create_done_node(&mut self, value: Value, secret: bool) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = DoneNode::new(value, secret);
        self.done_node_ids.push_back(output_id);
        self.nodes.insert(output_id, EngineNode::Done(node).into());

        output_id
    }

    pub fn create_native_function_node(
        &mut self,
        function_name: FunctionName,
        source: OutputId,
    ) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = NativeFunctionNode::new(function_name);
        let callback = Callback::native_function(output_id);
        self.nodes
            .insert(output_id, EngineNode::NativeFunction(node).into());
        self.add_callback(source, callback);
        output_id
    }

    pub fn create_resource_invoke_node(
        &mut self,
        token: String,
        inputs: HashMap<FieldName, OutputId>,
        version: String,
    ) -> OutputId {
        let operation =
            ResourceRequestOperation::Invoke(ResourceInvokeRequestOperation::new(token));
        self.create_register_or_read_resource_node(operation, inputs, version)
    }

    pub fn create_register_resource_node(
        &mut self,
        r#type: String,
        name: String,
        inputs: HashMap<FieldName, OutputId>,
        version: String,
    ) -> OutputId {
        let operation =
            ResourceRequestOperation::Register(RegisterResourceRequestOperation::new(r#type, name));
        self.create_register_or_read_resource_node(operation, inputs, version)
    }

    fn create_register_or_read_resource_node(
        &mut self,
        operation: ResourceRequestOperation,
        inputs: HashMap<FieldName, OutputId>,
        version: String,
    ) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = AbstractResourceNode::new(operation, inputs.keys().cloned().collect(), version);
        self.nodes
            .insert(output_id, EngineNode::RegisterResource(node).into());

        inputs.iter().for_each(|(field_name, source_output_id)| {
            let callback = Callback::create_resource(output_id, field_name.clone());
            self.add_callback(*source_output_id, callback);
        });

        output_id
    }

    pub fn create_extract_field(
        &mut self,
        field_name: FieldName,
        source_output_id: OutputId,
    ) -> OutputId {
        let in_preview = self.pulumi.is_in_preview();
        let output_id = Uuid::now_v7().into();
        let node = ExtractFieldNode::new(field_name, in_preview);
        let callback = Callback::extract_field(output_id);
        self.nodes
            .insert(output_id, EngineNode::ExtractField(node).into());
        self.add_callback(source_output_id, callback);
        output_id
    }

    pub fn create_combine_outputs(&mut self, outputs: Vec<OutputId>) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = CombineOutputsNode::new(outputs.len() as u32);

        outputs
            .iter()
            .enumerate()
            .for_each(|(index, source_output_id)| {
                let callback = Callback::combine_outputs(output_id, index as u32);
                self.add_callback(*source_output_id, callback);
            });
        self.nodes
            .insert(output_id, EngineNode::CombineOutputs(node).into());
        output_id
    }

    pub fn get_config_value(&mut self, name: &str, key: &str) -> Option<ConfigValue> {
        match self.config.get(name, key) {
            None => None,
            Some(RawConfigValue::PlainText(value)) => Some(ConfigValue::PlainText(value.clone())),
            Some(RawConfigValue::Secret(secret)) => {
                let value = Value::String(secret.clone());
                let output_id = self.create_done_node(value, true);
                Some(ConfigValue::Secret(output_id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Deref;

    use serde_json::Value;

    use crate::engine::{Engine, ForeignFunctionToInvoke};
    use crate::nodes::{Callback, DoneNode, NativeFunctionNode};
    use crate::pulumi::service::MockPulumiService;

    #[test]
    fn create_done_node_create_node_in_map() {
        let mut engine = Engine::new_without_configs(MockPulumiService::new());
        let value: Value = 1.into();
        let output_id = engine.create_done_node(value.clone(), false);

        assert_eq!(engine.done_node_ids, vec![output_id]);
        assert_eq!(
            engine.get_done(output_id).deref(),
            &DoneNode::create(value, false, Vec::new())
        );
    }

    #[test]
    fn create_native_function_node_create_node_in_map() {
        let mut engine = Engine::new_without_configs(MockPulumiService::new());
        let value: Value = 1.into();
        let done_node_output_id = engine.create_done_node(value.clone(), false);
        let native_node_output_id =
            engine.create_native_function_node("func".into(), done_node_output_id);

        assert_eq!(engine.done_node_ids, vec![done_node_output_id]);
        assert_eq!(engine.ready_foreign_function_ids, [].into());

        assert_eq!(
            engine.get_done(done_node_output_id).deref(),
            &DoneNode::create(
                value,
                false,
                vec![Callback::NativeFunction(native_node_output_id)]
            )
        );
        assert_eq!(
            engine.get_native_function(native_node_output_id).deref(),
            &NativeFunctionNode::new("func".into())
        );
    }

    mod native_function {
        use super::*;
        use crate::model::MaybeNodeValue;

        #[test]
        fn run_return_native_functions() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone(), false);
            let native_node_output_id =
                engine.create_native_function_node("func".into(), done_node_output_id);

            let result = engine.run(HashMap::new());
            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_node_output_id].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_node_output_id,
                    function_name: "func".into(),
                    value: 1.into(),
                }])
            );
        }

        #[test]
        fn sets_native_function_results() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone(), false);
            let native_node_output_id =
                engine.create_native_function_node("func".into(), done_node_output_id);
            let result_value: Value = 2.into();

            let result = engine.run(HashMap::from([(
                native_node_output_id,
                result_value.clone(),
            )]));

            assert_eq!(engine.ready_foreign_function_ids, [].into());
            assert_eq!(
                engine
                    .get_native_function(native_node_output_id)
                    .get_value(),
                &MaybeNodeValue::set_value(result_value, false)
            );
            assert_eq!(result, None);
        }

        #[test]
        fn native_function_passes_unknown_value_downstream() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone(), false);
            let native_node_output_id =
                engine.create_native_function_node("func".into(), done_node_output_id);
            let result_value: Value = 2.into();

            let result = engine.run(HashMap::from([(
                native_node_output_id,
                result_value.clone(),
            )]));

            assert_eq!(engine.ready_foreign_function_ids, [].into());
            assert_eq!(
                engine
                    .get_native_function(native_node_output_id)
                    .get_value(),
                &MaybeNodeValue::set_value(result_value, false)
            );
            assert_eq!(result, None);
        }

        #[test]
        fn native_function_can_be_run_from_another_native_function() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone(), false);
            let native_node_output_id_1 =
                engine.create_native_function_node("func".into(), done_node_output_id);
            let native_node_output_id_2 =
                engine.create_native_function_node("func2".into(), native_node_output_id_1);

            let result = engine.run(HashMap::from([(native_node_output_id_1, 2.into())]));

            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_node_output_id_2].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_node_output_id_2,
                    function_name: "func2".into(),
                    value: 2.into(),
                }])
            );
        }
    }

    mod extract_field {
        use crate::model::MaybeNodeValue::Set;
        use crate::model::NodeValue;

        use crate::nodes::Callback::NativeFunction;
        use crate::nodes::ExtractFieldNode;

        use super::*;

        #[test]
        fn extract_field_extract_field_from_map() {
            let mut mock = MockPulumiService::new();
            mock.expect_is_in_preview().times(1).returning(|| true);
            let mut engine = Engine::new_without_configs(mock);
            let value = Value::Object([("key".into(), 1.into())].into_iter().collect());
            let done_node_output_id = engine.create_done_node(value.clone(), false);
            let extract_field_node_output_id =
                engine.create_extract_field("key".into(), done_node_output_id);
            let native_function_node_output_id =
                engine.create_native_function_node("func".into(), extract_field_node_output_id);

            let result = engine.run(HashMap::new());

            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_function_node_output_id].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_function_node_output_id,
                    function_name: "func".into(),
                    value: 1.into(),
                }])
            );
            assert_eq!(
                engine
                    .get_extract_field_mut(extract_field_node_output_id)
                    .deref(),
                &ExtractFieldNode::create(
                    Set(NodeValue::exists(1.into(), false)),
                    "key".into(),
                    vec![NativeFunction(native_function_node_output_id)],
                    true,
                )
            );
        }
    }

    mod register_resource {
        use std::collections::{HashMap, HashSet};
        use std::ops::Deref;
        use std::sync::{Arc, OnceLock};

        use crate::engine::Engine;
        use crate::model::MaybeNodeValue;
        use crate::model::MaybeNodeValue::NotYetCalculated;
        use crate::nodes::{
            AbstractResourceNode, Callback, DoneNode, RegisterResourceRequestOperation,
            ResourceRequestOperation,
        };
        use crate::pulumi::service::{
            MockPulumiService, PerformResourceRequest, RegisterResourceResponse,
        };
        use mockall::predicate::{eq, function};
        use serde_json::json;

        #[test]
        fn should_create_required_nodes() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let done_node_output_id = engine.create_done_node(1.into(), false);
            let register_resource_node_output_id = engine.create_register_resource_node(
                "type".into(),
                "name".into(),
                HashMap::from([("input".into(), done_node_output_id)]),
                "1.0.0".into(),
            );

            assert_eq!(
                engine.get_done(done_node_output_id).deref(),
                &DoneNode::create(
                    1.into(),
                    false,
                    vec![Callback::create_resource(
                        register_resource_node_output_id,
                        "input".into(),
                    )],
                )
            );
            assert_eq!(
                engine
                    .get_create_resource(register_resource_node_output_id)
                    .deref(),
                &AbstractResourceNode::create(
                    NotYetCalculated,
                    ResourceRequestOperation::Register(RegisterResourceRequestOperation::new(
                        "type".into(),
                        "name".into()
                    )),
                    HashSet::from(["input".into()]),
                    HashMap::new(),
                    vec![],
                    "1.0.0".into()
                )
            );
        }

        #[test]
        fn should_create_resource_when_all_inputs_are_set() {
            let mut mock = MockPulumiService::new();

            let register_resource_node_output_id_once_cell = Arc::new(OnceLock::new());

            let register_resource_node_output_id_once_cell_2 =
                register_resource_node_output_id_once_cell.clone();
            mock.expect_perform_resource_operation()
                .times(1)
                .with(
                    function(move |output_id| {
                        output_id
                            == register_resource_node_output_id_once_cell_2
                                .deref()
                                .get()
                                .unwrap()
                    }),
                    eq(PerformResourceRequest {
                        operation: ResourceRequestOperation::Register(
                            RegisterResourceRequestOperation::new("type".into(), "name".into()),
                        ),
                        object: HashMap::from([("input".into(), Some(1.into()))]),
                        version: "1.0.0".into(),
                    }),
                )
                .returning(|_, _| ());

            let register_resource_node_output_id_once_cell_2 =
                register_resource_node_output_id_once_cell.clone();
            mock.expect_poll_resource_operations()
                .times(1)
                .with(function(move |output_ids| {
                    output_ids
                        == &([*register_resource_node_output_id_once_cell_2
                            .deref()
                            .get()
                            .unwrap()]
                        .into())
                }))
                .returning(|output_ids| {
                    let output_id = output_ids.iter().next().unwrap();

                    HashMap::from([(
                        *output_id,
                        RegisterResourceResponse {
                            outputs: HashMap::from([("output".into(), true.into())]),
                        },
                    )])
                });

            let mut engine = Engine::new_without_configs(mock);
            let done_node_output_id = engine.create_done_node(1.into(), false);
            let register_resource_node_output_id = engine.create_register_resource_node(
                "type".into(),
                "name".into(),
                HashMap::from([("input".into(), done_node_output_id)]),
                "1.0.0".into(),
            );
            register_resource_node_output_id_once_cell
                .set(register_resource_node_output_id)
                .unwrap();
            let result = engine.run(HashMap::new());
            assert_eq!(result, None);

            let output_node = engine.get_create_resource(register_resource_node_output_id);
            assert_eq!(
                output_node.get_value(),
                &MaybeNodeValue::set_value(json!({ "output": true }), false)
            );
        }
    }
    mod invoke_resource {
        use std::collections::{HashMap, HashSet};
        use std::ops::Deref;
        use std::sync::{Arc, OnceLock};

        use crate::engine::Engine;
        use crate::model::MaybeNodeValue;
        use crate::model::MaybeNodeValue::NotYetCalculated;
        use crate::nodes::{
            AbstractResourceNode, Callback, DoneNode, ResourceInvokeRequestOperation,
            ResourceRequestOperation,
        };
        use crate::pulumi::service::{
            MockPulumiService, PerformResourceRequest, RegisterResourceResponse,
        };
        use mockall::predicate::{eq, function};
        use serde_json::json;

        #[test]
        fn should_create_required_nodes() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let done_node_output_id = engine.create_done_node(1.into(), false);
            let invoke_resource_node_output_id = engine.create_resource_invoke_node(
                "token".into(),
                HashMap::from([("input".into(), done_node_output_id)]),
                "1.0.0".into(),
            );

            assert_eq!(
                engine.get_done(done_node_output_id).deref(),
                &DoneNode::create(
                    1.into(),
                    false,
                    vec![Callback::create_resource(
                        invoke_resource_node_output_id,
                        "input".into(),
                    )],
                )
            );
            assert_eq!(
                engine
                    .get_create_resource(invoke_resource_node_output_id)
                    .deref(),
                &AbstractResourceNode::create(
                    NotYetCalculated,
                    ResourceRequestOperation::Invoke(ResourceInvokeRequestOperation::new(
                        "token".into(),
                    )),
                    HashSet::from(["input".into()]),
                    HashMap::new(),
                    vec![],
                    "1.0.0".into()
                )
            );
        }

        #[test]
        fn should_create_resource_when_all_inputs_are_set() {
            let mut mock = MockPulumiService::new();

            let invoke_resource_node_output_id_once_cell = Arc::new(OnceLock::new());

            let invoke_resource_node_output_id_once_cell_2 =
                invoke_resource_node_output_id_once_cell.clone();
            mock.expect_perform_resource_operation()
                .times(1)
                .with(
                    function(move |output_id| {
                        output_id
                            == invoke_resource_node_output_id_once_cell_2
                                .deref()
                                .get()
                                .unwrap()
                    }),
                    eq(PerformResourceRequest {
                        operation: ResourceRequestOperation::Invoke(
                            ResourceInvokeRequestOperation::new("token".into()),
                        ),
                        object: HashMap::from([("input".into(), Some(1.into()))]),
                        version: "1.0.0".into(),
                    }),
                )
                .returning(|_, _| ());

            let invoke_resource_node_output_id_once_cell_2 =
                invoke_resource_node_output_id_once_cell.clone();
            mock.expect_poll_resource_operations()
                .times(1)
                .with(function(move |output_ids| {
                    output_ids
                        == &([*invoke_resource_node_output_id_once_cell_2
                            .deref()
                            .get()
                            .unwrap()]
                        .into())
                }))
                .returning(|output_ids| {
                    let output_id = output_ids.iter().next().unwrap();

                    HashMap::from([(
                        *output_id,
                        RegisterResourceResponse {
                            outputs: HashMap::from([("output".into(), true.into())]),
                        },
                    )])
                });

            let mut engine = Engine::new_without_configs(mock);
            let done_node_output_id = engine.create_done_node(1.into(), false);
            let invoke_resource_node_output_id = engine.create_resource_invoke_node(
                "token".into(),
                HashMap::from([("input".into(), done_node_output_id)]),
                "1.0.0".into(),
            );
            invoke_resource_node_output_id_once_cell
                .set(invoke_resource_node_output_id)
                .unwrap();
            let result = engine.run(HashMap::new());
            assert_eq!(result, None);

            let output_node = engine.get_create_resource(invoke_resource_node_output_id);
            assert_eq!(
                output_node.get_value(),
                &MaybeNodeValue::set_value(json!({ "output": true }), false)
            );
        }
    }

    mod combine_outputs {
        use super::*;
        use serde_json::json;

        #[test]
        fn combine_outputs_with_done_node() {
            let mut engine = Engine::new_without_configs(MockPulumiService::new());
            let value1 = engine.create_done_node("key".into(), false);
            let value2 = engine.create_done_node(1.into(), false);

            let combined = engine.create_combine_outputs(vec![value1, value2]);

            let native_function_node_output_id =
                engine.create_native_function_node("func".into(), combined);

            let result = engine.run(HashMap::new());

            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_function_node_output_id].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_function_node_output_id,
                    function_name: "func".into(),
                    value: json!(["key", 1])
                }])
            );
        }
    }

    mod register_outputs {
        use super::*;
        use mockall::predicate::eq;

        #[test]
        fn should_register_outputs() {
            let mut mock = MockPulumiService::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(HashMap::from([("output".into(), 1.into())])))
                .returning(|_| ());

            let mut engine = Engine::new_without_configs(mock);

            let output_id = engine.create_done_node(1.into(), false);
            engine.add_output("output".into(), output_id);

            let result = engine.run(HashMap::new());
            assert_eq!(result, None);
        }
    }

    mod config {
        use super::*;
        use crate::config::Config;
        use crate::engine::ConfigValue;
        use crate::model::NodeValue;

        use std::collections::HashSet;

        #[test]
        fn should_return_none_when_config_is_not_set() {
            let config = Config::new(
                HashMap::from([("name:key".to_string(), "value".to_string())]),
                HashSet::new(),
            );
            let mut engine = Engine::new(MockPulumiService::new(), config);
            let value = engine.get_config_value("name", "invalid_key");
            assert_eq!(value, None);
        }

        #[test]
        fn should_return_value_when_config_is_plain_text() {
            let config = Config::new(
                HashMap::from([("name:key".to_string(), "value".to_string())]),
                HashSet::new(),
            );
            let mut engine = Engine::new(MockPulumiService::new(), config);
            let value = engine.get_config_value("name", "key");
            assert_eq!(value, Some(ConfigValue::PlainText("value".to_string())));
        }

        #[test]
        fn should_return_secret_output_when_config_is_secret() {
            let config = Config::new(
                HashMap::from([("name:key".to_string(), "secret".to_string())]),
                HashSet::from(["name:key".to_string()]),
            );
            let mut engine = Engine::new(MockPulumiService::new(), config);
            let value = engine.get_config_value("name", "key");
            assert!(matches!(value, Some(ConfigValue::Secret(_))));
            let output_id = match value {
                Some(ConfigValue::Secret(output_id)) => output_id,
                _ => panic!("Expected secret output"),
            };
            let output = engine.get_done(output_id);
            assert_eq!(
                output.get_value(),
                &NodeValue::Exists {
                    value: "secret".into(),
                    secret: true,
                }
            );
        }
    }
}
