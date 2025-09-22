#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyAdaptiveProtectionConfig {
    /// Auto Deploy Config of this security policy
    #[builder(into)]
    #[serde(rename = "autoDeployConfigs")]
    pub r#auto_deploy_configs: Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigAutoDeployConfig>,
    /// Layer 7 DDoS Defense Config of this security policy
    #[builder(into)]
    #[serde(rename = "layer7DdosDefenseConfigs")]
    pub r#layer_7_ddos_defense_configs: Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig>,
}
