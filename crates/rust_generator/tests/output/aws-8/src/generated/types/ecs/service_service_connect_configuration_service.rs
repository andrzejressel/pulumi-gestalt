#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceServiceConnectConfigurationService {
    /// List of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1. See below.
    #[builder(into)]
    #[serde(rename = "clientAlias")]
    pub r#client_alias: Option<Vec<super::super::types::ecs::ServiceServiceConnectConfigurationServiceClientAlias>>,
    /// Name of the new AWS Cloud Map service that Amazon ECS creates for this Amazon ECS service.
    #[builder(into)]
    #[serde(rename = "discoveryName")]
    pub r#discovery_name: Option<String>,
    /// Port number for the Service Connect proxy to listen on.
    #[builder(into)]
    #[serde(rename = "ingressPortOverride")]
    pub r#ingress_port_override: Option<i32>,
    /// Name of one of the `portMappings` from all the containers in the task definition of this Amazon ECS service.
    #[builder(into)]
    #[serde(rename = "portName")]
    pub r#port_name: String,
    /// Configuration timeouts for Service Connect
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTimeout>>,
    /// Configuration for enabling Transport Layer Security (TLS)
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTls>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceServiceConnectConfigurationService {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "client_alias".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_alias,
                )
                .await,
            );
            map.insert(
                "discovery_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#discovery_name,
                )
                .await,
            );
            map.insert(
                "ingress_port_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ingress_port_override,
                )
                .await,
            );
            map.insert(
                "port_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port_name,
                )
                .await,
            );
            map.insert(
                "timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout,
                )
                .await,
            );
            map.insert(
                "tls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("client_alias") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_alias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#discovery_name: {
                        let field_value = match fields_map.get("discovery_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'discovery_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_port_override: {
                        let field_value = match fields_map.get("ingress_port_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_port_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_name: {
                        let field_value = match fields_map.get("port_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
