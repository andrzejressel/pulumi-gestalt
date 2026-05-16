#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduledQueryRulesAlertV2CriteriaDimension {
    /// Name of the dimension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Operator for dimension values. Possible values are `Exclude`,and `Include`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// List of dimension values. Use a wildcard `*` to collect all.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
