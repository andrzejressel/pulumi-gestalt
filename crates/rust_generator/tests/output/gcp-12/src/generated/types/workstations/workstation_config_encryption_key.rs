#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkstationConfigEncryptionKey {
    /// The name of the Google Cloud KMS encryption key.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: String,
    /// The service account to use with the specified KMS key.
    #[builder(into)]
    #[serde(rename = "kmsKeyServiceAccount")]
    pub r#kms_key_service_account: String,
}
