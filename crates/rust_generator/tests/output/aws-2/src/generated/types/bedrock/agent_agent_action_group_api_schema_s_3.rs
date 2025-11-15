#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentAgentActionGroupApiSchemaS3 {
    /// Name of the S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Option<String>,
    /// S3 object key containing the resource.
    #[builder(into)]
    #[serde(rename = "s3ObjectKey")]
    pub r#s_3_object_key: Option<String>,
}
