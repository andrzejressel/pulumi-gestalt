#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainAssociationCertificateSettings {
    /// DNS records for certificate verification in a space-delimited format (`<record> CNAME <target>`).
    #[builder(into)]
    #[serde(rename = "certificateVerificationDnsRecord")]
    pub r#certificate_verification_dns_record: Option<String>,
    /// The Amazon resource name (ARN) for the custom certificate.
    /// Required when `type` is `CUSTOM`.
    #[builder(into)]
    #[serde(rename = "customCertificateArn")]
    pub r#custom_certificate_arn: Option<String>,
    /// The certificate type.
    /// Valid values are `AMPLIFY_MANAGED` and `CUSTOM`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
