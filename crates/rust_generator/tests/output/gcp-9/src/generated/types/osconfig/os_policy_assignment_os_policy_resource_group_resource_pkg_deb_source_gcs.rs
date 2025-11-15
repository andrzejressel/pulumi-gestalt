#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkgDebSourceGcs {
    /// Bucket of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Generation number of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "generation")]
    pub r#generation: Option<i32>,
    /// Name of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
}
