#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainEncryptionAtRest {
    /// Enabled disabled toggle for off-peak update window
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// KMS key id used to encrypt data at rest.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: String,
}
