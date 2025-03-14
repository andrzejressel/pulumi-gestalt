#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCryptoKeyVersionsVersion {
    /// The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<String>,
    /// The `id` of the Google Cloud Platform CryptoKey to which the key version belongs. This is also the `id` field of the 
    /// `gcp.kms.CryptoKey` resource/datasource.
    #[builder(into)]
    #[serde(rename = "cryptoKey")]
    pub r#crypto_key: Box<String>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "protectionLevel")]
    pub r#protection_level: Box<String>,
    #[builder(into)]
    #[serde(rename = "publicKeys")]
    pub r#public_keys: Box<Vec<super::super::types::kms::GetCryptoKeyVersionsVersionPublicKey>>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<i32>,
}
