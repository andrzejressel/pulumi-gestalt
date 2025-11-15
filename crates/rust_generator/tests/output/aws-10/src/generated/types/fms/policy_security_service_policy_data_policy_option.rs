#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicySecurityServicePolicyDataPolicyOption {
    /// Defines the deployment model to use for the firewall policy. Documented below.
    #[builder(into)]
    #[serde(rename = "networkFirewallPolicy")]
    pub r#network_firewall_policy: Option<Box<super::super::types::fms::PolicySecurityServicePolicyDataPolicyOptionNetworkFirewallPolicy>>,
    #[builder(into)]
    #[serde(rename = "thirdPartyFirewallPolicy")]
    pub r#third_party_firewall_policy: Option<Box<super::super::types::fms::PolicySecurityServicePolicyDataPolicyOptionThirdPartyFirewallPolicy>>,
}
