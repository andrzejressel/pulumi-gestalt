#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduledQueryRulesLogCriteriaDimension {
    /// Name of the dimension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Operator for dimension values, - 'Include'. Defaults to `Include`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    /// List of dimension values.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
