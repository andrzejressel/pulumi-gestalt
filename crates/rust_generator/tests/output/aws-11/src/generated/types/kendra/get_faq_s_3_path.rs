#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFaqS3Path {
    /// Name of the S3 bucket that contains the file.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Name of the file.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
}
