#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames {
    /// A list of alternative DNS names (FQDNs) identified by the Certificate.
    #[builder(into)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Option<Vec<String>>,
    /// A list of email addresses identified by this Certificate.
    #[builder(into)]
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    /// A list of User Principal Names identified by the Certificate.
    #[builder(into)]
    #[serde(rename = "upns")]
    pub r#upns: Option<Vec<String>>,
}
