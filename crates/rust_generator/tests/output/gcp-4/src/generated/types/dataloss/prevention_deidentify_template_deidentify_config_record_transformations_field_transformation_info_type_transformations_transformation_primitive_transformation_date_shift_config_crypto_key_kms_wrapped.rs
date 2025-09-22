#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfigCryptoKeyKmsWrapped {
    /// The resource name of the KMS CryptoKey to use for unwrapping.
    #[builder(into)]
    #[serde(rename = "cryptoKeyName")]
    pub r#crypto_key_name: String,
    /// The wrapped data crypto key.
    /// A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "wrappedKey")]
    pub r#wrapped_key: String,
}
