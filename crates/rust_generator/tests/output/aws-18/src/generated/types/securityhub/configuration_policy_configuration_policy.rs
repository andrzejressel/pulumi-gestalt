#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationPolicyConfigurationPolicy {
    /// A list that defines which security standards are enabled in the configuration policy. It must be defined if `service_enabled` is set to true.
    #[builder(into)]
    #[serde(rename = "enabledStandardArns")]
    pub r#enabled_standard_arns: Option<Vec<String>>,
    /// Defines which security controls are enabled in the configuration policy and any customizations to parameters affecting them. See below.
    #[builder(into)]
    #[serde(rename = "securityControlsConfiguration")]
    pub r#security_controls_configuration: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration>>,
    /// Indicates whether Security Hub is enabled in the policy.
    #[builder(into)]
    #[serde(rename = "serviceEnabled")]
    pub r#service_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationPolicyConfigurationPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "enabled_standard_arns",
                    &self.r#enabled_standard_arns,
                ),
                to_pulumi_object_field(
                    "security_controls_configuration",
                    &self.r#security_controls_configuration,
                ),
                to_pulumi_object_field(
                    "service_enabled",
                    &self.r#service_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationPolicyConfigurationPolicy {
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
                    r#enabled_standard_arns: {
                        let field_value = match fields_map.get("enabled_standard_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled_standard_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_controls_configuration: {
                        let field_value = match fields_map.get("security_controls_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_controls_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_enabled: {
                        let field_value = match fields_map.get("service_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
