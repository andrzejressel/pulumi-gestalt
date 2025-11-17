#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReportGroupExportConfigS3Destination {
    /// The name of the S3 bucket where the raw data of a report are exported.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// A boolean value that specifies if the results of a report are encrypted.
    /// **Note: the API does not currently allow setting encryption as disabled**
    #[builder(into)]
    #[serde(rename = "encryptionDisabled")]
    pub r#encryption_disabled: Option<bool>,
    /// The encryption key for the report's encrypted raw data. The KMS key ARN.
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: String,
    /// The type of build output artifact to create. Valid values are: `NONE` (default) and `ZIP`.
    #[builder(into)]
    #[serde(rename = "packaging")]
    pub r#packaging: Option<String>,
    /// The path to the exported report's raw data results.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}
