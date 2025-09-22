#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceS3ReferenceDataSource {
    /// The ARN of the S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: String,
    /// The object key name containing the reference data.
    #[builder(into)]
    #[serde(rename = "fileKey")]
    pub r#file_key: String,
}
