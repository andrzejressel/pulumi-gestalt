#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertifiateCertificatePolicyX509CertificateProperties {
    /// A list of Extended/Enhanced Key Usages.
    #[builder(into, default)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Box<Option<Vec<String>>>,
    /// A list of uses associated with this Key. Possible values include `cRLSign`, `dataEncipherment`, `decipherOnly`, `digitalSignature`, `encipherOnly`, `keyAgreement`, `keyCertSign`, `keyEncipherment` and `nonRepudiation` and are case-sensitive.
    #[builder(into)]
    #[serde(rename = "keyUsages")]
    pub r#key_usages: Box<Vec<String>>,
    /// The Certificate's Subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<String>,
    /// A `subject_alternative_names` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Option<super::super::types::keyvault::CertifiateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames>>,
    /// The Certificates Validity Period in Months.
    #[builder(into)]
    #[serde(rename = "validityInMonths")]
    pub r#validity_in_months: Box<i32>,
}
