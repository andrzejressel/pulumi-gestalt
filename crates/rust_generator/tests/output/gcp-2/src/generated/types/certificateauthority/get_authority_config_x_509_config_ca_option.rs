#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAuthorityConfigX509ConfigCaOption {
    /// When true, the "CA" in Basic Constraints extension will be set to true.
    #[builder(into)]
    #[serde(rename = "isCa")]
    pub r#is_ca: Box<bool>,
    /// Refers to the "path length constraint" in Basic Constraints extension. For a CA certificate, this value describes the depth of
    /// subordinate CA certificates that are allowed. If this value is less than 0, the request will fail. Setting the value to 0
    /// requires setting 'zero_max_issuer_path_length = true'.
    #[builder(into)]
    #[serde(rename = "maxIssuerPathLength")]
    pub r#max_issuer_path_length: Box<i32>,
    /// When true, the "CA" in Basic Constraints extension will be set to false.
    /// If both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate.
    #[builder(into)]
    #[serde(rename = "nonCa")]
    pub r#non_ca: Box<bool>,
    /// When true, the "path length constraint" in Basic Constraints extension will be set to 0.
    /// If both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,
    /// the max path length will be omitted from the CA certificate.
    #[builder(into)]
    #[serde(rename = "zeroMaxIssuerPathLength")]
    pub r#zero_max_issuer_path_length: Box<bool>,
}
