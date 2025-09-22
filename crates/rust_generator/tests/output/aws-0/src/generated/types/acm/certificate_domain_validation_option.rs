#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateDomainValidationOption {
    /// Fully qualified domain name (FQDN) in the certificate.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Option<String>,
    /// The name of the DNS record to create to validate the certificate
    #[builder(into)]
    #[serde(rename = "resourceRecordName")]
    pub r#resource_record_name: Option<String>,
    /// The type of DNS record to create
    #[builder(into)]
    #[serde(rename = "resourceRecordType")]
    pub r#resource_record_type: Option<String>,
    /// The value the DNS record needs to have
    #[builder(into)]
    #[serde(rename = "resourceRecordValue")]
    pub r#resource_record_value: Option<String>,
}
