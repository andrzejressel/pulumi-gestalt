#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TrustAnchorSourceSourceData {
    /// The ARN of an ACM Private Certificate Authority.
    #[builder(into)]
    #[serde(rename = "acmPcaArn")]
    pub r#acm_pca_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "x509CertificateData")]
    pub r#x_509_certificate_data: Option<String>,
}
