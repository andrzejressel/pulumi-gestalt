#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableImportTableS3BucketSource {
    /// The S3 bucket that is being imported from.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// The account number of the S3 bucket that is being imported from.
    #[builder(into)]
    #[serde(rename = "bucketOwner")]
    pub r#bucket_owner: Option<String>,
    /// The key prefix shared by all S3 Objects that are being imported.
    #[builder(into)]
    #[serde(rename = "keyPrefix")]
    pub r#key_prefix: Option<String>,
}
