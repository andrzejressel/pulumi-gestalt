#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAlertRuleTemplateNrtTemplate {
    /// The description of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The query of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// The alert severity of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: String,
    /// A list of categories of attacks by which to classify the rule.
    #[builder(into)]
    #[serde(rename = "tactics")]
    pub r#tactics: Vec<String>,
}
