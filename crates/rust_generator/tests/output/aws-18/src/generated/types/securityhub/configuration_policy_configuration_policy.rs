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
