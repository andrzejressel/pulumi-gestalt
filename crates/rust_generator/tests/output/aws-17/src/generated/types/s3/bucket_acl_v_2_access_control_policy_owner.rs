#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketAclV2AccessControlPolicyOwner {
    /// Display name of the owner.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// ID of the owner.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
}
