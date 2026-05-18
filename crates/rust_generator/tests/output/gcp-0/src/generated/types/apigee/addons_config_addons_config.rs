#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AddonsConfigAddonsConfig {
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "advancedApiOpsConfig")]
    pub r#advanced_api_ops_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigAdvancedApiOpsConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "apiSecurityConfig")]
    pub r#api_security_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigApiSecurityConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "connectorsPlatformConfig")]
    pub r#connectors_platform_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigConnectorsPlatformConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "integrationConfig")]
    pub r#integration_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigIntegrationConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "monetizationConfig")]
    pub r#monetization_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigMonetizationConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AddonsConfigAddonsConfig {
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
                    "advanced_api_ops_config",
                    &self.r#advanced_api_ops_config,
                ),
                to_pulumi_object_field(
                    "api_security_config",
                    &self.r#api_security_config,
                ),
                to_pulumi_object_field(
                    "connectors_platform_config",
                    &self.r#connectors_platform_config,
                ),
                to_pulumi_object_field(
                    "integration_config",
                    &self.r#integration_config,
                ),
                to_pulumi_object_field(
                    "monetization_config",
                    &self.r#monetization_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AddonsConfigAddonsConfig {
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
                    r#advanced_api_ops_config: {
                        let field_value = match fields_map.get("advanced_api_ops_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_api_ops_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_security_config: {
                        let field_value = match fields_map.get("api_security_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_security_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connectors_platform_config: {
                        let field_value = match fields_map.get("connectors_platform_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectors_platform_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integration_config: {
                        let field_value = match fields_map.get("integration_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'integration_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monetization_config: {
                        let field_value = match fields_map.get("monetization_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'monetization_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
