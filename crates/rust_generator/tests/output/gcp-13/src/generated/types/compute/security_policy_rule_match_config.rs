#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyRuleMatchConfig {
    /// CIDR IP address range. Maximum number of srcIpRanges allowed is 10.
    #[builder(into)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Option<Vec<String>>,
}
