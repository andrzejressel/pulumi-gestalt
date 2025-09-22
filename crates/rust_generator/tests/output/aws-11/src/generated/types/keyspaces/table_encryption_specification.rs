#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableEncryptionSpecification {
    /// The Amazon Resource Name (ARN) of the customer managed KMS key.
    #[builder(into)]
    #[serde(rename = "kmsKeyIdentifier")]
    pub r#kms_key_identifier: Option<String>,
    /// The encryption option specified for the table. Valid values: `AWS_OWNED_KMS_KEY`, `CUSTOMER_MANAGED_KMS_KEY`. The default value is `AWS_OWNED_KMS_KEY`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
