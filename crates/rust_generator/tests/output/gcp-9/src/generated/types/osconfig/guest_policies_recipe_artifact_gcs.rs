#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeArtifactGcs {
    /// Bucket of the Google Cloud Storage object. Given an example URL: https://storage.googleapis.com/my-bucket/foo/bar#1234567
    /// this value would be my-bucket.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// Must be provided if allowInsecure is false. Generation number of the Google Cloud Storage object.
    /// https://storage.googleapis.com/my-bucket/foo/bar#1234567 this value would be 1234567.
    #[builder(into)]
    #[serde(rename = "generation")]
    pub r#generation: Option<i32>,
    /// Name of the Google Cloud Storage object. Given an example URL: https://storage.googleapis.com/my-bucket/foo/bar#1234567
    /// this value would be foo/bar.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Option<String>,
}
