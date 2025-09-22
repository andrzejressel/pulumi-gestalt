#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterDatabaseEncryption {
    /// The key to use to encrypt/decrypt secrets.
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: String,
    /// ENCRYPTED or DECRYPTED.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
}
