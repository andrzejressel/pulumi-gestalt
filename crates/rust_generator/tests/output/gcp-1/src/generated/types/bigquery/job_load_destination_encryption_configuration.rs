#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobLoadDestinationEncryptionConfiguration {
    /// Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table.
    /// The BigQuery Service Account associated with your project requires access to this encryption key.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: String,
    /// (Output)
    /// Describes the Cloud KMS encryption key version used to protect destination BigQuery table.
    #[builder(into)]
    #[serde(rename = "kmsKeyVersion")]
    pub r#kms_key_version: Option<String>,
}
