#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidateFileGcs {
    /// Bucket of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Generation number of the Cloud Storage object.
    #[builder(into, default)]
    #[serde(rename = "generation")]
    pub r#generation: Box<Option<i32>>,
    /// Name of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
