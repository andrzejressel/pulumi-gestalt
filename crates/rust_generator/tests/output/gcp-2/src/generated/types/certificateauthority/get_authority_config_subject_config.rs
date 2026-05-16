#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAuthorityConfigSubjectConfig {
    /// The subject alternative name fields.
    #[builder(into)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectConfigSubjectAltName>,
    /// Contains distinguished name fields such as the location and organization.
    #[builder(into)]
    #[serde(rename = "subjects")]
    pub r#subjects: Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectConfigSubject>,
}
