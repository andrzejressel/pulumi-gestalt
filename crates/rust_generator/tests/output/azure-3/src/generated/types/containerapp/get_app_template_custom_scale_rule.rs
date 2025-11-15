#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAppTemplateCustomScaleRule {
    #[builder(into)]
    #[serde(rename = "authentications")]
    pub r#authentications: Vec<super::super::types::containerapp::GetAppTemplateCustomScaleRuleAuthentication>,
    #[builder(into)]
    #[serde(rename = "customRuleType")]
    pub r#custom_rule_type: String,
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: std::collections::HashMap<String, String>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
