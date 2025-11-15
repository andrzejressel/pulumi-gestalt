#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateAuthorityRevocationConfigurationOcspConfiguration {
    /// Boolean value that specifies whether a custom OCSP responder is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// CNAME specifying a customized OCSP domain. Note: The value of the CNAME must not include a protocol prefix such as "http://" or "https://".
    #[builder(into)]
    #[serde(rename = "ocspCustomCname")]
    pub r#ocsp_custom_cname: Option<String>,
}
