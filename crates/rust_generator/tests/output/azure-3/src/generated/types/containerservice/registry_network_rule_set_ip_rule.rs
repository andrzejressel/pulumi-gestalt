#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistryNetworkRuleSetIpRule {
    /// The behaviour for requests matching this rule. At this time the only supported value is `Allow`
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The CIDR block from which requests will match the rule.
    #[builder(into)]
    #[serde(rename = "ipRange")]
    pub r#ip_range: String,
}
