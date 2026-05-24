#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodeType {
    /// A `application_ports` block as defined below.
    #[builder(into)]
    pub r#application_ports: Option<Box<super::super::types::servicefabric::ClusterNodeTypeApplicationPorts>>,
    /// The capacity tags applied to the nodes in the node type, the cluster resource manager uses these tags to understand how much resource a node has.
    #[builder(into)]
    pub r#capacities: Option<std::collections::BTreeMap<String, String>>,
    /// The Port used for the Client Endpoint for this Node Type.
    #[builder(into)]
    pub r#client_endpoint_port: i32,
    /// The Durability Level for this Node Type. Possible values include `Bronze`, `Gold` and `Silver`. Defaults to `Bronze`.
    #[builder(into)]
    pub r#durability_level: Option<String>,
    /// A `ephemeral_ports` block as defined below.
    #[builder(into)]
    pub r#ephemeral_ports: Option<Box<super::super::types::servicefabric::ClusterNodeTypeEphemeralPorts>>,
    /// The Port used for the HTTP Endpoint for this Node Type.
    #[builder(into)]
    pub r#http_endpoint_port: i32,
    /// The number of nodes for this Node Type.
    #[builder(into)]
    pub r#instance_count: i32,
    /// Is this the Primary Node Type?
    #[builder(into)]
    pub r#is_primary: bool,
    /// Should this node type run only stateless services?
    #[builder(into)]
    pub r#is_stateless: Option<bool>,
    /// Does this node type span availability zones?
    #[builder(into)]
    pub r#multiple_availability_zones: Option<bool>,
    /// The name of the Node Type.
    #[builder(into)]
    pub r#name: String,
    /// The placement tags applied to nodes in the node type, which can be used to indicate where certain services (workload) should run.
    #[builder(into)]
    pub r#placement_properties: Option<std::collections::BTreeMap<String, String>>,
    /// The Port used for the Reverse Proxy Endpoint for this Node Type. Changing this will upgrade the cluster.
    #[builder(into)]
    pub r#reverse_proxy_endpoint_port: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodeType {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "applicationPorts",
                    &self.r#application_ports,
                ),
                to_pulumi_object_field(
                    "capacities",
                    &self.r#capacities,
                ),
                to_pulumi_object_field(
                    "clientEndpointPort",
                    &self.r#client_endpoint_port,
                ),
                to_pulumi_object_field(
                    "durabilityLevel",
                    &self.r#durability_level,
                ),
                to_pulumi_object_field(
                    "ephemeralPorts",
                    &self.r#ephemeral_ports,
                ),
                to_pulumi_object_field(
                    "httpEndpointPort",
                    &self.r#http_endpoint_port,
                ),
                to_pulumi_object_field(
                    "instanceCount",
                    &self.r#instance_count,
                ),
                to_pulumi_object_field(
                    "isPrimary",
                    &self.r#is_primary,
                ),
                to_pulumi_object_field(
                    "isStateless",
                    &self.r#is_stateless,
                ),
                to_pulumi_object_field(
                    "multipleAvailabilityZones",
                    &self.r#multiple_availability_zones,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "placementProperties",
                    &self.r#placement_properties,
                ),
                to_pulumi_object_field(
                    "reverseProxyEndpointPort",
                    &self.r#reverse_proxy_endpoint_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodeType {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#application_ports: {
                        let field_value = match fields_map.get("applicationPorts") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationPorts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capacities: {
                        let field_value = match fields_map.get("capacities") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_endpoint_port: {
                        let field_value = match fields_map.get("clientEndpointPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientEndpointPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#durability_level: {
                        let field_value = match fields_map.get("durabilityLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'durabilityLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_ports: {
                        let field_value = match fields_map.get("ephemeralPorts") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeralPorts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_endpoint_port: {
                        let field_value = match fields_map.get("httpEndpointPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpEndpointPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_count: {
                        let field_value = match fields_map.get("instanceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_primary: {
                        let field_value = match fields_map.get("isPrimary") {
                            Some(value) => value,
                            None => bail!("Missing field 'isPrimary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_stateless: {
                        let field_value = match fields_map.get("isStateless") {
                            Some(value) => value,
                            None => bail!("Missing field 'isStateless' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiple_availability_zones: {
                        let field_value = match fields_map.get("multipleAvailabilityZones") {
                            Some(value) => value,
                            None => bail!("Missing field 'multipleAvailabilityZones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_properties: {
                        let field_value = match fields_map.get("placementProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'placementProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reverse_proxy_endpoint_port: {
                        let field_value = match fields_map.get("reverseProxyEndpointPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'reverseProxyEndpointPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
