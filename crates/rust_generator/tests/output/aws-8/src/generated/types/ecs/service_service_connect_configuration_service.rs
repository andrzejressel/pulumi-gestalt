#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceServiceConnectConfigurationService {
    /// List of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1. See below.
    #[builder(into)]
    pub r#client_alias: Option<Vec<super::super::types::ecs::ServiceServiceConnectConfigurationServiceClientAlias>>,
    /// Name of the new AWS Cloud Map service that Amazon ECS creates for this Amazon ECS service.
    #[builder(into)]
    pub r#discovery_name: Option<String>,
    /// Port number for the Service Connect proxy to listen on.
    #[builder(into)]
    pub r#ingress_port_override: Option<i32>,
    /// Name of one of the `portMappings` from all the containers in the task definition of this Amazon ECS service.
    #[builder(into)]
    pub r#port_name: String,
    /// Configuration timeouts for Service Connect
    #[builder(into)]
    pub r#timeout: Option<Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTimeout>>,
    /// Configuration for enabling Transport Layer Security (TLS)
    #[builder(into)]
    pub r#tls: Option<Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTls>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceServiceConnectConfigurationService {
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
                    "clientAlias",
                    &self.r#client_alias,
                ),
                to_pulumi_object_field(
                    "discoveryName",
                    &self.r#discovery_name,
                ),
                to_pulumi_object_field(
                    "ingressPortOverride",
                    &self.r#ingress_port_override,
                ),
                to_pulumi_object_field(
                    "portName",
                    &self.r#port_name,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
                to_pulumi_object_field(
                    "tls",
                    &self.r#tls,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceServiceConnectConfigurationService {
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
                    r#client_alias: {
                        let field_value = match fields_map.get("clientAlias") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientAlias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#discovery_name: {
                        let field_value = match fields_map.get("discoveryName") {
                            Some(value) => value,
                            None => bail!("Missing field 'discoveryName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_port_override: {
                        let field_value = match fields_map.get("ingressPortOverride") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingressPortOverride' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_name: {
                        let field_value = match fields_map.get("portName") {
                            Some(value) => value,
                            None => bail!("Missing field 'portName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls: {
                        let field_value = match fields_map.get("tls") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
