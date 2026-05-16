#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxClusterCode {
    /// Unique name for the S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: String,
    /// Full S3 path (excluding bucket) to the .zip file that contains the code to be loaded onto the cluster when it’s started.
    #[builder(into)]
    #[serde(rename = "s3Key")]
    pub r#s_3_key: String,
    /// Version of an S3 Object.
    #[builder(into)]
    #[serde(rename = "s3ObjectVersion")]
    pub r#s_3_object_version: Option<String>,
}
