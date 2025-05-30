#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicySecurityServicePolicyDataPolicyOptionThirdPartyFirewallPolicy {
    /// Defines the deployment model to use for the third-party firewall policy. Valid values are `CENTRALIZED` and `DISTRIBUTED`.
    #[builder(into, default)]
    #[serde(rename = "firewallDeploymentModel")]
    pub r#firewall_deployment_model: Box<Option<String>>,
}
