#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecListenerTlValidation {
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTlValidationSubjectAlternativeName>>,
    #[builder(into)]
    #[serde(rename = "trusts")]
    pub r#trusts: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTlValidationTrust>>,
}
