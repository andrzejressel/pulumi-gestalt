#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHostnameSsl {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[builder(into)]
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Option<String>,
    #[builder(into)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Option<String>,
    /// If a custom uploaded certificate is used.
    #[builder(into)]
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Option<String>,
    /// The key for a custom uploaded certificate.
    #[builder(into)]
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<String>,
    /// Domain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// SSL/TLS settings for the certificate.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Option<Vec<super::types::CustomHostnameSslSetting>>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Level of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    #[builder(into)]
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Option<Vec<super::types::CustomHostnameSslValidationError>>,
    #[builder(into)]
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Option<Vec<super::types::CustomHostnameSslValidationRecord>>,
    /// Indicates whether the certificate covers a wildcard.
    #[builder(into)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Option<bool>,
}
