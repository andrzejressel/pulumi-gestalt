#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeystoresAliasesSelfSignedCertSubjectAlternativeDnsNames {
    /// Subject Alternative Name
    #[builder(into)]
    #[serde(rename = "subjectAlternativeName")]
    pub r#subject_alternative_name: Option<String>,
}
