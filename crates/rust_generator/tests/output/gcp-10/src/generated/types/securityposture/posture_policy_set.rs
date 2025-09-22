#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySet {
    /// Description of the policy set.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// List of security policy
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policies")]
    pub r#policies: Vec<super::super::types::securityposture::PosturePolicySetPolicy>,
    /// ID of the policy set.
    #[builder(into)]
    #[serde(rename = "policySetId")]
    pub r#policy_set_id: String,
}
