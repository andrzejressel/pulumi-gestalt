#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceServiceConnectConfigurationServiceTlsIssuerCertAuthority {
    /// ARN of the `aws.acmpca.CertificateAuthority` used to create the TLS Certificates.
    #[builder(into)]
    #[serde(rename = "awsPcaAuthorityArn")]
    pub r#aws_pca_authority_arn: String,
}
