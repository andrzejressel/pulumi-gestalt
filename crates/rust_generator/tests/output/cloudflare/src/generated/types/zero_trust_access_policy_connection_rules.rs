#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustAccessPolicyConnectionRules {
    /// The SSH-specific rules that define how users may connect to the targets secured by your application.
    #[builder(into)]
    #[serde(rename = "ssh")]
    pub r#ssh: Box<super::types::ZeroTrustAccessPolicyConnectionRulesSsh>,
}
