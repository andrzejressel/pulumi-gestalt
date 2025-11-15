#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertProcessingRuleActionGroupConditionSeverity {
    /// The operator for a given condition. Possible values are `Equals` and `NotEquals`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Specifies list of values to match for a given condition. Possible values are `Sev0`, `Sev1`, `Sev2`, `Sev3`, and `Sev4`.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
