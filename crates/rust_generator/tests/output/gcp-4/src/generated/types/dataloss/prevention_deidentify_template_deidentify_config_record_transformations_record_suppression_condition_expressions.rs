#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsRecordSuppressionConditionExpressions {
    /// Conditions to apply to the expression.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsRecordSuppressionConditionExpressionsConditions>>,
    /// The operator to apply to the result of conditions. Default and currently only supported value is AND.
    /// Default value is `AND`.
    /// Possible values are: `AND`.
    #[builder(into)]
    #[serde(rename = "logicalOperator")]
    pub r#logical_operator: Option<String>,
}
