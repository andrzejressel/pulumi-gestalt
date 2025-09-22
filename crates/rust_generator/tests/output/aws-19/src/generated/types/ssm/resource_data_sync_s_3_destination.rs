#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceDataSyncS3Destination {
    /// Name of S3 bucket where the aggregated data is stored.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// ARN of an encryption key for a destination in Amazon S3.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
    /// Prefix for the bucket.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Region with the bucket targeted by the Resource Data Sync.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// A supported sync format. Only JsonSerDe is currently supported. Defaults to JsonSerDe.
    #[builder(into)]
    #[serde(rename = "syncFormat")]
    pub r#sync_format: Option<String>,
}
