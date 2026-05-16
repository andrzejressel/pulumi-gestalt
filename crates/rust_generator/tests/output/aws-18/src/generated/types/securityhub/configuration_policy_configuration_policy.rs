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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enabled_standard_arns".to_string(), self.r#enabled_standard_arns.to_pulumi_value().await);
            map.insert("security_controls_configuration".to_string(), self.r#security_controls_configuration.to_pulumi_value().await);
            map.insert("service_enabled".to_string(), self.r#service_enabled.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationPolicyConfigurationPolicy {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#enabled_standard_arns: {
                        let field_value = match fields_map.get("enabled_standard_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled_standard_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#security_controls_configuration: {
                        let field_value = match fields_map.get("security_controls_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_controls_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_enabled: {
                        let field_value = match fields_map.get("service_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
