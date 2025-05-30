use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::model::{FieldName, OutputId};
use crate::nodes::ResourceRequestOperation;
use crate::pulumi::service::{PulumiService, RegisterResourceResponse};
use crate::pulumi::service_impl::RequestType::{Invoke, Register};
use crate::{PerformResourceRequest, PulumiConnector};
use log::error;
use prost::Message;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct};
use pulumi_gestalt_proto::pulumi::pulumirpc;
use serde_json::{Number, Value};

enum RequestType {
    Invoke,
    Register,
}

struct Result {
    request_type: RequestType,
}

impl Result {
    fn new(request_type: RequestType) -> Result {
        Result { request_type }
    }
}

pub struct PulumiServiceImpl {
    connector: Box<dyn PulumiConnector>,
    expected_results: RefCell<HashMap<OutputId, Result>>,
    is_in_preview: bool,
}

impl PulumiServiceImpl {
    pub fn new(
        connector: impl PulumiConnector + 'static,
        is_in_preview: bool,
    ) -> PulumiServiceImpl {
        Self {
            connector: Box::new(connector),
            is_in_preview,
            expected_results: RefCell::new(HashMap::new()),
        }
    }
}

impl PulumiService for PulumiServiceImpl {
    fn is_in_preview(&self) -> bool {
        self.is_in_preview
    }

    fn get_root_resource(&self) -> String {
        unimplemented!()
    }

    fn register_outputs(&self, outputs: HashMap<FieldName, Value>) {
        if !self.is_in_preview {
            let object = Self::create_protobuf_struct(
                outputs.into_iter().map(|(k, v)| (k, Some(v))).collect(),
            );

            let request = pulumirpc::RegisterResourceOutputsRequest {
                urn: "INVALID".to_string(), // Will be fixed in grpc_connection
                outputs: Some(object),
            };

            self.connector.register_outputs(request);
        }
    }

    fn perform_resource_operation(&self, output_id: OutputId, request: PerformResourceRequest) {
        match request.operation {
            ResourceRequestOperation::Register(register) => {
                {
                    self.expected_results
                        .borrow_mut()
                        .insert(output_id, Result::new(Register));
                }

                let object = Self::create_protobuf_struct(request.object);

                let req = pulumirpc::RegisterResourceRequest {
                    r#type: register.r#type.clone(),
                    name: register.name.clone(),
                    parent: "".to_string(),
                    custom: true,
                    object: Some(object),
                    protect: false,
                    dependencies: vec![],
                    provider: "".to_string(),
                    property_dependencies: Default::default(),
                    // property_dependencies: HashMap::from(
                    //     [("value".to_string(), register_resource_request::PropertyDependencies { urns: vec!["test".to_string()] })]
                    // ),
                    delete_before_replace: false,
                    version: request.version,
                    ignore_changes: vec![],
                    accept_secrets: true,
                    additional_secret_outputs: vec![],
                    alias_ur_ns: vec![],
                    import_id: "".to_string(),
                    custom_timeouts: None,
                    delete_before_replace_defined: false,
                    supports_partial_values: false,
                    remote: false,
                    accept_resources: false,
                    providers: Default::default(),
                    replace_on_changes: vec![],
                    plugin_download_url: "".to_string(),
                    plugin_checksums: Default::default(),
                    retain_on_delete: false,
                    aliases: vec![],
                    deleted_with: "".to_string(),
                    alias_specs: true,
                    source_position: None,
                    supports_result_reporting: false,
                    transforms: vec![],
                    package_ref: "".to_string(),
                };

                self.connector.register_resource(output_id.to_string(), req);
            }
            ResourceRequestOperation::Invoke(invoke) => {
                {
                    self.expected_results
                        .borrow_mut()
                        .insert(output_id, Result::new(Invoke));
                }

                let object = Self::create_protobuf_struct(request.object);

                let req = pulumirpc::ResourceInvokeRequest {
                    tok: invoke.token,
                    args: Some(object),
                    provider: "".to_string(),
                    version: request.version,
                    accept_resources: false,
                    plugin_download_url: "".to_string(),
                    plugin_checksums: Default::default(),
                    source_position: None,
                    package_ref: "".to_string(),
                };

                self.connector.resource_invoke(output_id.to_string(), req);
            }
        }
    }

    fn poll_resource_operations(
        &self,
        _register_ids: &HashSet<OutputId>,
    ) -> HashMap<OutputId, RegisterResourceResponse> {
        let results = { self.connector.get_created_resources() };

        let mut map = HashMap::new();

        for (output_id, response) in results {
            let output_id = output_id.into();
            let expected_results_ref = self.expected_results.borrow();
            let expected_results = expected_results_ref.get(&output_id).unwrap();

            let object = match expected_results.request_type {
                Invoke => {
                    let response = pulumirpc::InvokeResponse::decode(&*response).unwrap();
                    // TODO: Failures
                    response.r#return.unwrap_or(Struct::default())
                }
                Register => {
                    let response = pulumirpc::RegisterResourceResponse::decode(&*response).unwrap();
                    response.object.unwrap_or(Struct::default())
                }
            };

            let result = Self::protoc_object_to_json_map(object);

            map.insert(output_id, RegisterResourceResponse { outputs: result });
        }

        map
    }
}

const UNKNOWN_VALUE: &str = "04da6b54-80e4-46f7-96ec-b56ff0331ba9";

impl PulumiServiceImpl {
    fn protoc_object_to_json_map(o: Struct) -> HashMap<FieldName, Value> {
        o.fields
            .iter()
            .flat_map(|(k, v)| {
                let v = Self::protobuf_to_json(v);
                Some((k.into(), v))
            })
            .collect::<HashMap<_, _>>()
    }

    fn create_protobuf_struct(fields: HashMap<FieldName, Option<Value>>) -> Struct {
        let pairs = fields
            .iter()
            .map(|(name, value)| {
                let v = match value {
                    None => prost_types::Value {
                        kind: Some(Kind::StringValue(UNKNOWN_VALUE.into())),
                    },
                    Some(value) => Self::json_to_protobuf(value),
                };
                (name.as_string().clone(), v)
            })
            .collect::<Vec<_>>();

        Struct {
            fields: BTreeMap::from_iter(pairs),
        }
    }

    fn json_to_protobuf(json: &Value) -> prost_types::Value {
        match json {
            Value::Null => prost_types::Value {
                kind: Some(prost_types::value::Kind::NullValue(0)),
            },
            Value::Bool(b) => prost_types::Value {
                kind: Some(prost_types::value::Kind::BoolValue(*b)),
            },
            Value::Number(n) => prost_types::Value {
                kind: Some(prost_types::value::Kind::NumberValue(n.as_f64().unwrap())),
            },
            Value::String(s) => prost_types::Value {
                kind: Some(prost_types::value::Kind::StringValue(s.clone())),
            },
            Value::Array(arr) => {
                let list_value = ListValue {
                    values: arr.iter().map(Self::json_to_protobuf).collect(),
                };
                prost_types::Value {
                    kind: Some(prost_types::value::Kind::ListValue(list_value)),
                }
            }
            Value::Object(obj) => {
                let struct_value = Struct {
                    fields: obj
                        .iter()
                        .map(|(k, v)| (k.clone(), Self::json_to_protobuf(v)))
                        .collect(),
                };
                prost_types::Value {
                    kind: Some(prost_types::value::Kind::StructValue(struct_value)),
                }
            }
        }
    }

    fn protobuf_to_json(protobuf: &prost_types::Value) -> Value {
        match &protobuf.kind {
            None => {
                error!("Unknown kind in protobuf value");
                panic!("Unknown kind in protobuf value");
            }
            Some(Kind::NullValue(_)) => Value::Null,
            Some(Kind::NumberValue(n)) => {
                if n.fract() == 0.0 {
                    Value::Number(Number::from(*n as i64))
                } else {
                    Value::Number(Number::from_f64(*n).unwrap())
                }
            }
            Some(Kind::StringValue(s)) => Value::String(s.clone()),
            Some(Kind::BoolValue(b)) => Value::Bool(*b),
            Some(Kind::StructValue(s)) => Value::Object(
                s.fields
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::protobuf_to_json(v)))
                    .collect(),
            ),
            Some(Kind::ListValue(l)) => {
                Value::Array(l.values.iter().map(Self::protobuf_to_json).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use prost_types::Struct;

    use crate::pulumi::service_impl::PulumiServiceImpl;

    #[test]
    fn protoc_object_to_messagepack() {
        let s = Struct {
            fields: BTreeMap::from([
                (
                    "field1".into(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(1.5)),
                    },
                ),
                (
                    "field2".into(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(2.5)),
                    },
                ),
            ]),
        };

        let result = PulumiServiceImpl::protoc_object_to_json_map(s);

        assert_eq!(
            result,
            HashMap::from([("field1".into(), 1.5.into()), ("field2".into(), 2.5.into())])
        );
    }

    mod register_resource {}

    mod register_resource_poll {}
}
