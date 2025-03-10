#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LbCertificateDomainValidationRecord {
    /// The domain name (e.g., example.com) for your SSL/TLS certificate.
    #[builder(into, default)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "resourceRecordName")]
    pub r#resource_record_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "resourceRecordType")]
    pub r#resource_record_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "resourceRecordValue")]
    pub r#resource_record_value: Box<Option<String>>,
}
