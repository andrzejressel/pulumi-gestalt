#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InventoryDestinationBucket {
    /// ID of the account that owns the destination bucket. Recommended to be set to prevent problems if the destination bucket ownership changes.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// Amazon S3 bucket ARN of the destination.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: String,
    /// Contains the type of server-side encryption to use to encrypt the inventory (documented below).
    #[builder(into)]
    #[serde(rename = "encryption")]
    pub r#encryption: Option<Box<super::super::types::s3::InventoryDestinationBucketEncryption>>,
    /// Specifies the output format of the inventory results. Can be `CSV`, [`ORC`](https://orc.apache.org/) or [`Parquet`](https://parquet.apache.org/).
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    /// Prefix that is prepended to all inventory results.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
