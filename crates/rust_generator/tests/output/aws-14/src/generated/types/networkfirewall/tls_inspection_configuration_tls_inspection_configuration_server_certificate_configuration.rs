#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration {
    /// ARN of the imported certificate authority (CA) certificate within Certificate Manager (ACM) to use for outbound SSL/TLS inspection. See [Using SSL/TLS certificates with TLS inspection configurations](https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection-certificate-requirements.html) for limitations on CA certificates.
    #[builder(into)]
    #[serde(rename = "certificateAuthorityArn")]
    pub r#certificate_authority_arn: Option<String>,
    /// Check Certificate Revocation Status block. Detailed below.
    #[builder(into)]
    #[serde(rename = "checkCertificateRevocationStatus")]
    pub r#check_certificate_revocation_status: Box<Option<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationCheckCertificateRevocationStatus>>,
    /// Scope block. Detailed below.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope>>,
    /// Server certificates to use for inbound SSL/TLS inspection. See [Using SSL/TLS certificates with TLS inspection configurations](https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection-certificate-requirements.html).
    #[builder(into)]
    #[serde(rename = "serverCertificates")]
    pub r#server_certificates: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate>>,
}
