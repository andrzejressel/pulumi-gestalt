#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionEventingConfigAdditionalVariableEncryptionKeyValue {
    /// The [KMS key name] with which the content of the Operation is encrypted. The
    /// expected format: projects/*/locations/*/keyRings/*/cryptoKeys/*.
    /// Will be empty string if google managed.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
    /// Type of Encryption Key
    /// Possible values are: `GOOGLE_MANAGED`, `CUSTOMER_MANAGED`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
