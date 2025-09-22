#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainSingleSignOn {
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    #[builder(into)]
    #[serde(rename = "userAssignment")]
    pub r#user_assignment: Option<String>,
}
