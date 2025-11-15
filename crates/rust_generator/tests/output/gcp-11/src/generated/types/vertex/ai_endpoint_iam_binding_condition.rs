#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiEndpointIamBindingCondition {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: String,
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}
