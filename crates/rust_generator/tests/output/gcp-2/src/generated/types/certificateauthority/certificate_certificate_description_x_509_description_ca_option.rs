#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificateDescriptionX509DescriptionCaOption {
    /// When true, the "CA" in Basic Constraints extension will be set to true.
    #[builder(into)]
    #[serde(rename = "isCa")]
    pub r#is_ca: Option<bool>,
    /// Refers to the "path length constraint" in Basic Constraints extension. For a CA certificate, this value describes the depth of
    /// subordinate CA certificates that are allowed. If this value is less than 0, the request will fail.
    #[builder(into)]
    #[serde(rename = "maxIssuerPathLength")]
    pub r#max_issuer_path_length: Option<i32>,
}
