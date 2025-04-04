#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomHostnameSsl {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[builder(into, default)]
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<String>>,
    /// If a custom uploaded certificate is used.
    #[builder(into, default)]
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Box<Option<String>>,
    /// The key for a custom uploaded certificate.
    #[builder(into, default)]
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<String>>,
    /// Domain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// SSL/TLS settings for the certificate.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<super::types::CustomHostnameSslSetting>>>,
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Level of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Box<Option<Vec<super::types::CustomHostnameSslValidationError>>>,
    #[builder(into, default)]
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Box<Option<Vec<super::types::CustomHostnameSslValidationRecord>>>,
    /// Indicates whether the certificate covers a wildcard.
    #[builder(into, default)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}
