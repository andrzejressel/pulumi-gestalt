#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateConfigSubjectConfig {
    /// Contains distinguished name fields such as the location and organization.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<super::super::types::certificateauthority::CertificateConfigSubjectConfigSubject>,
    /// The subject alternative name fields.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subjectAltName")]
    pub r#subject_alt_name: Option<Box<super::super::types::certificateauthority::CertificateConfigSubjectConfigSubjectAltName>>,
}
