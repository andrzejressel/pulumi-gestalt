#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkFirewallPolicyRuleTargetSecureTag {
    /// Name of the secure tag, created with TagManager's TagValue API.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// (Output)
    /// State of the secure tag, either EFFECTIVE or INEFFECTIVE. A secure tag is INEFFECTIVE when it is deleted or its network is deleted.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
