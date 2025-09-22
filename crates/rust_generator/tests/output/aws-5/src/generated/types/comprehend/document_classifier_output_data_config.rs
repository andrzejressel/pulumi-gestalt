#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DocumentClassifierOutputDataConfig {
    /// KMS Key used to encrypt the output documents.
    /// Can be a KMS Key ID, a KMS Key ARN, a KMS Alias name, or a KMS Alias ARN.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Full path for the output documents.
    #[builder(into)]
    #[serde(rename = "outputS3Uri")]
    pub r#output_s_3_uri: Option<String>,
    /// Destination path for the output documents.
    /// The full path to the output file will be returned in `output_s3_uri`.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
}
