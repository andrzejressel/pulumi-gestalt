#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TlsInspectionConfigurationEncryptionConfiguration {
    /// ARN of the Amazon Web Services Key Management Service (KMS) customer managed key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<String>,
    /// Type of KMS key to use for encryption of your Network Firewall resources. Valid values: `AWS_OWNED_KMS_KEY`, `CUSTOMER_KMS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
