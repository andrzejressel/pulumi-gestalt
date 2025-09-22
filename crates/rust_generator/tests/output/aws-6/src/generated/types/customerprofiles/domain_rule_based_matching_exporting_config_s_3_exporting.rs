#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainRuleBasedMatchingExportingConfigS3Exporting {
    /// The name of the S3 bucket where Identity Resolution Jobs write result files.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: String,
    /// The S3 key name of the location where Identity Resolution Jobs write result files.
    #[builder(into)]
    #[serde(rename = "s3KeyName")]
    pub r#s_3_key_name: Option<String>,
}
