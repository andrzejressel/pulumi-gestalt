#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
