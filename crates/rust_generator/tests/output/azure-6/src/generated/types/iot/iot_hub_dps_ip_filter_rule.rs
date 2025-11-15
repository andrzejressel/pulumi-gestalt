#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IotHubDpsIpFilterRule {
    /// The desired action for requests captured by this rule. Possible values are `Accept`, `Reject`
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The IP address range in CIDR notation for the rule.
    #[builder(into)]
    #[serde(rename = "ipMask")]
    pub r#ip_mask: String,
    /// The name of the filter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Target for requests captured by this rule. Possible values are `all`, `deviceApi` and `serviceApi`.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}
