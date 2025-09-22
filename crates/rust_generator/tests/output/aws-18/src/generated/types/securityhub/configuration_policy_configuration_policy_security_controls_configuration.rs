#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration {
    /// A list of security controls that are disabled in the configuration policy Security Hub enables all other controls (including newly released controls) other than the listed controls. Conflicts with `enabled_control_identifiers`.
    #[builder(into)]
    #[serde(rename = "disabledControlIdentifiers")]
    pub r#disabled_control_identifiers: Option<Vec<String>>,
    /// A list of security controls that are enabled in the configuration policy. Security Hub disables all other controls (including newly released controls) other than the listed controls. Conflicts with `disabled_control_identifiers`.
    #[builder(into)]
    #[serde(rename = "enabledControlIdentifiers")]
    pub r#enabled_control_identifiers: Option<Vec<String>>,
    /// A list of control parameter customizations that are included in a configuration policy. Include multiple blocks to define multiple control custom parameters. See below.
    #[builder(into)]
    #[serde(rename = "securityControlCustomParameters")]
    pub r#security_control_custom_parameters: Option<Vec<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter>>,
}
