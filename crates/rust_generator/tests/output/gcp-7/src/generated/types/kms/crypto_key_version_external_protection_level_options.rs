#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CryptoKeyVersionExternalProtectionLevelOptions {
    /// The path to the external key material on the EKM when using EkmConnection e.g., "v0/my/key". Set this field instead of externalKeyUri when using an EkmConnection.
    #[builder(into)]
    #[serde(rename = "ekmConnectionKeyPath")]
    pub r#ekm_connection_key_path: Option<String>,
    /// The URI for an external resource that this CryptoKeyVersion represents.
    #[builder(into)]
    #[serde(rename = "externalKeyUri")]
    pub r#external_key_uri: Option<String>,
}
