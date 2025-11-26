use crate::constants::UNKNOWN_VALUE;
use anyhow::{Context, Error, Result};
use log::error;
use prost::Message;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct};
use pulumi_gestalt_domain::connector::{
    PulumiConnector, RegisterOutputsRequest, RegisterResourceResult, ResourceInvokeResult,
};
use pulumi_gestalt_domain::{ExistingNodeValue, FieldName, NodeValue, ResourceFields};
use pulumi_gestalt_proto::pulumi::pulumirpc::engine_client::EngineClient;
use pulumi_gestalt_proto::pulumi::pulumirpc::resource_monitor_client::ResourceMonitorClient;
use pulumi_gestalt_proto::pulumi::pulumirpc::{
    GetRootResourceRequest, InvokeResponse, RegisterResourceOutputsRequest, ResourceInvokeRequest,
};
use pulumi_gestalt_proto::pulumi::pulumirpc::{
    RegisterResourceRequest, RegisterResourceResponse, SetRootResourceRequest,
};
use serde_json::{Number, Value, json};
use std::collections::{BTreeMap, HashMap};
use tonic::transport::Channel;

pub struct RealPulumiConnector {
    resource_monitor_client: ResourceMonitorClient<Channel>,
    root_resource: String,
    in_preview: bool,
}

impl RealPulumiConnector {
    pub async fn new(
        monitor_url: String,
        engine_url: String,
        pulumi_project: String,
        pulumi_stack: String,
        in_preview: bool,
    ) -> Result<Self> {
        let resource_monitor_client =
            ResourceMonitorClient::connect(format!("tcp://{monitor_url}")).await?;
        let engine_client = EngineClient::connect(format!("tcp://{engine_url}")).await?;

        Self::create_root_stack(
            resource_monitor_client.clone(),
            engine_client.clone(),
            pulumi_project.clone(),
            pulumi_stack.clone(),
        )
        .await?;
        let root_resource =
            RealPulumiConnector::get_root_resource_async(engine_url.clone()).await?;

        let s = Self {
            resource_monitor_client,
            root_resource,
            in_preview,
        };

        Ok(s)
    }

    pub async fn send_register_resource_request(
        &self,
        request: RegisterResourceRequest,
    ) -> Result<RegisterResourceResponse> {
        let monitor = self.resource_monitor_client.clone();
        Self::send_register_resource_request_inner(request, monitor).await
    }

    pub async fn send_resource_invoke_request(
        &self,
        request: ResourceInvokeRequest,
    ) -> Result<InvokeResponse> {
        let monitor = self.resource_monitor_client.clone();
        Self::send_resource_invoke_request_inner(request, monitor).await
    }

    pub async fn register_resource_outputs(
        &self,
        request: RegisterResourceOutputsRequest,
    ) -> Result<()> {
        let mut monitor = self.resource_monitor_client.clone();
        let _ = monitor.register_resource_outputs(request).await?;
        Ok(())
    }

    pub async fn create_root_stack(
        monitor: ResourceMonitorClient<Channel>,
        engine: EngineClient<Channel>,
        pulumi_project: String,
        pulumi_stack: String,
    ) -> std::result::Result<(), Error> {
        let request = RegisterResourceRequest {
            r#type: "pulumi:pulumi:Stack".to_string(),
            name: format!("{}-{}", pulumi_project, pulumi_stack),
            custom: false,
            ..Default::default()
        };

        let result = Self::register_async(request, monitor)
            .await
            .context("Failed to register root stack")?;

        let urn = RegisterResourceResponse::decode(&mut result.as_slice())
            .context("Failed to decode register resource response")?
            .urn;
        Self::set_root_resource_async(urn, engine)
            .await
            .context("Failed to set root resource")?;

        Ok(())
    }

    async fn send_register_resource_request_inner(
        request: RegisterResourceRequest,
        mut monitor: ResourceMonitorClient<Channel>,
    ) -> Result<RegisterResourceResponse> {
        let result = monitor.register_resource(request).await?;
        Ok(result.into_inner())
    }

    async fn send_resource_invoke_request_inner(
        request: ResourceInvokeRequest,
        mut monitor: ResourceMonitorClient<Channel>,
    ) -> Result<InvokeResponse> {
        let result = monitor.invoke(request).await?;
        Ok(result.into_inner())
    }

    async fn register_async(
        request: RegisterResourceRequest,
        mut monitor: ResourceMonitorClient<Channel>,
    ) -> Result<Vec<u8>> {
        let result = monitor
            .register_resource(request)
            .await
            .context("Failed to register resource")?;

        Ok(result.get_ref().encode_to_vec())
    }

    async fn set_root_resource_async(urn: String, mut engine: EngineClient<Channel>) -> Result<()> {
        let request = SetRootResourceRequest { urn };

        let _ = engine
            .set_root_resource(request)
            .await
            .context("Failed to set root resource")?;

        Ok(())
    }

    async fn get_root_resource_async(engine_url: String) -> Result<String> {
        let mut client = EngineClient::connect(format!("tcp://{engine_url}")).await?;

        let request = GetRootResourceRequest {};

        let result = client.get_root_resource(request).await?;

        Ok(result.get_ref().urn.clone())
    }
}

#[async_trait::async_trait]
impl PulumiConnector for RealPulumiConnector {
    async fn register_resource(
        &self,
        req: pulumi_gestalt_domain::connector::RegisterResourceRequest,
    ) -> RegisterResourceResult {
        let req = RegisterResourceRequest {
            r#type: req.r#type,
            name: req.name,
            object: Some(create_protobuf_struct(req.object)),
            version: req.version,
            custom: true,
            ..Default::default()
        };
        let response = self
            .send_register_resource_request(req)
            .await
            .context("Failed to send register resource request")
            .unwrap();

        let obj = response.object;

        if (!self.in_preview) {
            panic!("{:?}", obj);
        }
        
        let map = match obj {
            Some(s) => create_map_of_node_values(s),
            None => HashMap::new(),
        };

        RegisterResourceResult {
            fields: ResourceFields {
                object: map,
                is_in_preview: self.in_preview,
            },
        }
    }

    async fn resource_invoke(
        &self,
        req: pulumi_gestalt_domain::connector::ResourceInvokeRequest,
    ) -> ResourceInvokeResult {
        let req = ResourceInvokeRequest {
            tok: req.token,
            args: Some(create_protobuf_struct(req.object)),
            version: req.version,
            ..Default::default()
        };
        let response = self
            .send_resource_invoke_request(req)
            .await
            .context("Failed to send resource invoke request")
            .unwrap();

        let obj = response.r#return;

        let map = match obj {
            Some(s) => create_map_of_node_values(s),
            None => HashMap::new(),
        };

        ResourceInvokeResult {
            fields: ResourceFields {
                object: map,
                is_in_preview: self.in_preview,
            },
        }
    }

    async fn register_outputs(&self, req: RegisterOutputsRequest) -> () {
        let req = RegisterResourceOutputsRequest {
            urn: self.root_resource.clone(),
            outputs: Some(create_protobuf_struct(req.outputs)),
        };
        self.register_resource_outputs(req)
            .await
            .context("Failed to register resource outputs")
            .unwrap();
    }
}

fn create_map_of_node_values(s: Struct) -> HashMap<FieldName, ExistingNodeValue> {
    s.fields
        .into_iter()
        .map(|(k, v)| {
            let json_value = protobuf_to_json(&v);

            let node_value = match &json_value {

                // Value::Object(obj) if obj.contains_key(crate::constants::SPECIAL_SIG_KEY) => {
                //     let secret_value = obj
                //         .get(crate::constants::SECRET_VALUE_NAME)
                //         .cloned()
                //         .unwrap_or(Value::Null);
                //     ExistingNodeValue {
                //         value: secret_value,
                //         secret: true,
                //     }
                // }
                _ => ExistingNodeValue {
                    value: json_value,
                    secret: false,
                },
            };
            (FieldName::from(k), node_value)
        })
        .collect()
}

fn create_protobuf_struct(fields: HashMap<FieldName, NodeValue>) -> Struct {
    let pairs = fields
        .into_iter()
        .map(|(name, value)| {
            let v = match value {
                NodeValue::Nothing => prost_types::Value {
                    kind: Some(Kind::StringValue(UNKNOWN_VALUE.into())),
                },
                NodeValue::Exists(ExistingNodeValue {
                    value,
                    secret: true,
                }) => {
                    let value = json!({
                        crate::constants::SPECIAL_SIG_KEY: crate::constants::SPECIAL_SECRET_SIG,
                        crate::constants::SECRET_VALUE_NAME: value
                    });
                    json_to_protobuf(value)
                }
                NodeValue::Exists(ExistingNodeValue {
                    value,
                    secret: false,
                }) => json_to_protobuf(value),
            };
            (name.get_inner(), v)
        })
        .collect::<Vec<_>>();

    Struct {
        fields: BTreeMap::from_iter(pairs),
    }
}

fn json_to_protobuf(json: Value) -> prost_types::Value {
    match json {
        Value::Null => prost_types::Value {
            kind: Some(Kind::NullValue(0)),
        },
        Value::Bool(b) => prost_types::Value {
            kind: Some(Kind::BoolValue(b)),
        },
        Value::Number(n) => prost_types::Value {
            kind: Some(Kind::NumberValue(n.as_f64().unwrap())),
        },
        Value::String(s) => prost_types::Value {
            kind: Some(Kind::StringValue(s.clone())),
        },
        Value::Array(arr) => {
            let list_value = ListValue {
                values: arr.into_iter().map(json_to_protobuf).collect(),
            };
            prost_types::Value {
                kind: Some(Kind::ListValue(list_value)),
            }
        }
        Value::Object(obj) => {
            let struct_value = Struct {
                fields: obj
                    .into_iter()
                    .map(|(k, v)| (k.clone(), json_to_protobuf(v)))
                    .collect(),
            };
            prost_types::Value {
                kind: Some(Kind::StructValue(struct_value)),
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
                .map(|(k, v)| (k.clone(), protobuf_to_json(v)))
                .collect(),
        ),
        Some(Kind::ListValue(l)) => Value::Array(l.values.iter().map(protobuf_to_json).collect()),
    }
}

#[cfg(test)]
mod tests {
    use crate::output_id::OutputId;
    use crate::pulumi_state::PulumiState;
    use crate::test_server::{MyResourceEngineServer, MyResourceMonitorServer};
    use anyhow::Result;
    use std::collections::{BTreeMap, HashMap};

    use crate::constants::{SECRET_VALUE_NAME, SPECIAL_SIG_KEY};
    use crate::real_pulumi_connector::{create_map_of_node_values, create_protobuf_struct};
    use prost_types::Struct;
    use pulumi_gestalt_domain::{ExistingNodeValue, FieldName, NodeValue};
    use pulumi_gestalt_proto::pulumi::pulumirpc::RegisterResourceRequest;
    use pulumi_gestalt_proto::pulumi::pulumirpc::engine_server::EngineServer;
    use pulumi_gestalt_proto::pulumi::pulumirpc::resource_monitor_server::ResourceMonitorServer;
    use std::time::Instant;
    use tokio::net::TcpListener;
    use tonic::codegen::tokio_stream;
    use tonic::transport::Server;

    #[tokio::test]
    async fn test() -> Result<()> {
        let monitor_listener = TcpListener::bind("127.0.0.1:0").await?;
        let engine_listener = TcpListener::bind("127.0.0.1:0").await?;
        let monitor_addr = monitor_listener.local_addr()?;
        let engine_addr = engine_listener.local_addr()?;

        let monitor_server = Server::builder()
            .add_service(ResourceMonitorServer::new(MyResourceMonitorServer {}))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(
                monitor_listener,
            ));

        let engine_server = Server::builder()
            .add_service(EngineServer::new(MyResourceEngineServer {}))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(
                engine_listener,
            ));

        tokio::spawn(monitor_server);
        tokio::spawn(engine_server);

        let mut pulumi_state = PulumiState::new(
            monitor_addr.to_string(),
            engine_addr.to_string(),
            "project".to_string(),
            "stack".to_string(),
        )
        .await?;

        let output_id_1 = OutputId::new("1".into());
        let output_id_2 = OutputId::new("2".into());
        let output_id_3 = OutputId::new("3".into());

        pulumi_state.send_register_resource_request(output_id_1, create_request("test1"));
        pulumi_state.send_register_resource_request(output_id_2, create_request("test2"));
        pulumi_state.send_register_resource_request(output_id_3, create_request("test3"));

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let start = Instant::now();
        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 2);
        assert!(start.elapsed().as_secs() <= 1);

        let start = Instant::now();
        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 1);
        assert!(start.elapsed().as_secs() <= 3);
        assert!(start.elapsed().as_secs() >= 1);

        let start = Instant::now();
        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 0);
        assert!(start.elapsed().as_secs() <= 1);

        Ok(())
    }

    fn create_request(name: &str) -> RegisterResourceRequest {
        RegisterResourceRequest {
            r#type: "".to_string(),
            name: name.into(),
            parent: "".to_string(),
            custom: false,
            object: None,
            protect: false,
            dependencies: vec![],
            provider: "".to_string(),
            property_dependencies: Default::default(),
            delete_before_replace: false,
            version: "".to_string(),
            ignore_changes: vec![],
            accept_secrets: false,
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
            alias_specs: false,
            source_position: None,
            supports_result_reporting: false,
            transforms: vec![],
            package_ref: "".to_string(),
        }
    }

    #[test]
    fn should_convert_node_values_to_protobuf() {
        let normal_value = NodeValue::exists("normal", false);
        let secret_value = NodeValue::exists("secret", true);
        let nothing_value = NodeValue::Nothing;

        let all_values: HashMap<FieldName, NodeValue> = HashMap::from([
            ("normal".into(), normal_value),
            ("secret".into(), secret_value),
            ("nothing".into(), nothing_value),
        ]);

        let protobuf_struct = create_protobuf_struct(all_values);
        assert_eq!(
            protobuf_struct,
            Struct {
                fields: BTreeMap::from([
                    (
                        "normal".to_string(),
                        prost_types::Value {
                            kind: Some(prost_types::value::Kind::StringValue("normal".to_string())),
                        },
                    ),
                    (
                        "secret".to_string(),
                        prost_types::Value {
                            kind: Some(prost_types::value::Kind::StructValue(Struct {
                                fields: BTreeMap::from([
                                    (
                                        SPECIAL_SIG_KEY.to_string(),
                                        prost_types::Value {
                                            kind: Some(prost_types::value::Kind::StringValue(
                                                crate::constants::SPECIAL_SECRET_SIG.to_string(),
                                            )),
                                        },
                                    ),
                                    (
                                        SECRET_VALUE_NAME.to_string(),
                                        prost_types::Value {
                                            kind: Some(prost_types::value::Kind::StringValue(
                                                "secret".to_string(),
                                            )),
                                        },
                                    ),
                                ]),
                            })),
                        },
                    ),
                    (
                        "nothing".to_string(),
                        prost_types::Value {
                            kind: Some(prost_types::value::Kind::StringValue(
                                crate::constants::UNKNOWN_VALUE.to_string(),
                            )),
                        },
                    ),
                ]),
            }
        );
    }

    #[test]
    fn should_convert_protobuf_struct_to_existing_nodes() {
        let protobuf_struct = Struct {
            fields: BTreeMap::from([
                (
                    "normal".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StringValue("normal".to_string())),
                    },
                ),
                (
                    "secret".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(Struct {
                            fields: BTreeMap::from([
                                (
                                    SPECIAL_SIG_KEY.to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StringValue(
                                            crate::constants::SPECIAL_SECRET_SIG.to_string(),
                                        )),
                                    },
                                ),
                                (
                                    SECRET_VALUE_NAME.to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StringValue(
                                            "secret".to_string(),
                                        )),
                                    },
                                ),
                            ]),
                        })),
                    },
                ),
            ]),
        };

        let node_values = create_map_of_node_values(protobuf_struct);
        assert_eq!(
            node_values,
            HashMap::from([
                ("normal".into(), ExistingNodeValue::new("normal", false)),
                ("secret".into(), ExistingNodeValue::new("secret", true))
            ])
        );
    }
}
