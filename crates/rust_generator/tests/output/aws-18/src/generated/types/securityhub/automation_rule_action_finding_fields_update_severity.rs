#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutomationRuleActionFindingFieldsUpdateSeverity {
    /// The severity value of the finding. The allowed values are the following `INFORMATIONAL`, `LOW`, `MEDIUM`, `HIGH` and `CRITICAL`.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// The native severity as defined by the AWS service or integrated partner product that generated the finding.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Option<f64>,
}
