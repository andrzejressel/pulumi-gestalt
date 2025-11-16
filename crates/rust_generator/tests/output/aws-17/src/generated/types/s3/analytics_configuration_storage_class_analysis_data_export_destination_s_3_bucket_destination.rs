#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsConfigurationStorageClassAnalysisDataExportDestinationS3BucketDestination {
    /// Account ID that owns the destination bucket.
    #[builder(into)]
    #[serde(rename = "bucketAccountId")]
    pub r#bucket_account_id: Option<String>,
    /// ARN of the destination bucket.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: String,
    /// Output format of exported analytics data. Allowed values: `CSV`. Default value: `CSV`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// Prefix to append to exported analytics data.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
