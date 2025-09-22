#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrganizationPolicyListPolicy {
    /// or `deny` - (Optional) One or the other must be set.
    #[builder(into)]
    #[serde(rename = "allow")]
    pub r#allow: Option<Box<super::super::types::folder::OrganizationPolicyListPolicyAllow>>,
    /// One or the other must be set.
    #[builder(into)]
    #[serde(rename = "deny")]
    pub r#deny: Option<Box<super::super::types::folder::OrganizationPolicyListPolicyDeny>>,
    /// If set to true, the values from the effective Policy of the parent resource
    /// are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy.
    /// 
    /// The `allow` or `deny` blocks support:
    #[builder(into)]
    #[serde(rename = "inheritFromParent")]
    pub r#inherit_from_parent: Option<bool>,
    /// The Google Cloud Console will try to default to a configuration that matches the value specified in this field.
    #[builder(into)]
    #[serde(rename = "suggestedValue")]
    pub r#suggested_value: Option<String>,
}
