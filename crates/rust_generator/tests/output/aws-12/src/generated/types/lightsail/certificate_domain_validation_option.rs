#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateDomainValidationOption {
    /// A domain name for which the certificate should be issued.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "resourceRecordName")]
    pub r#resource_record_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "resourceRecordType")]
    pub r#resource_record_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "resourceRecordValue")]
    pub r#resource_record_value: Option<String>,
}
