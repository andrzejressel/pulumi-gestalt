#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomSslCustomSslOptions {
    /// Method of building intermediate certificate chain. A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[builder(into)]
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Option<String>,
    /// Certificate certificate and the intermediate(s).
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    /// Specifies the region where your private key can be held locally. Available values: `us`, `eu`, `highest_security`.
    #[builder(into)]
    #[serde(rename = "geoRestrictions")]
    pub r#geo_restrictions: Option<String>,
    /// Certificate's private key.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    /// Whether to enable support for legacy clients which do not include SNI in the TLS handshake. Available values: `legacy_custom`, `sni_custom`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
