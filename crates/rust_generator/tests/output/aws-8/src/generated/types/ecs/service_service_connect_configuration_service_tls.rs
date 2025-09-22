#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceServiceConnectConfigurationServiceTls {
    /// Details of the certificate authority which will issue the certificate.
    #[builder(into)]
    #[serde(rename = "issuerCertAuthority")]
    pub r#issuer_cert_authority: Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTlsIssuerCertAuthority>,
    /// KMS key used to encrypt the private key in Secrets Manager.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Option<String>,
    /// ARN of the IAM Role that's associated with the Service Connect TLS.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
}
