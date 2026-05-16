#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
