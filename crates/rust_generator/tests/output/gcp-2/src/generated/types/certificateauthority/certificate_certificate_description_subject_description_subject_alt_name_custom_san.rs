#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan {
    /// (Output)
    /// Indicates whether or not the name constraints are marked critical.
    #[builder(into)]
    #[serde(rename = "critical")]
    pub r#critical: Option<bool>,
    /// (Output)
    /// Describes how some of the technical fields in a certificate should be populated.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "obectIds")]
    pub r#obect_ids: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSanObectId>>,
    /// The value of this X.509 extension. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
