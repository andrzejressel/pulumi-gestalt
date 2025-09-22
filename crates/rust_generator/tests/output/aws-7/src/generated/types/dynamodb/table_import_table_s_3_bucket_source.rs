#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
