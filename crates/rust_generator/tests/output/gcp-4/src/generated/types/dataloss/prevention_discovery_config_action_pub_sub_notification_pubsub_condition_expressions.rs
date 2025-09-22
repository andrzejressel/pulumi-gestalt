#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressions {
    /// Conditions to apply to the expression
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressionsCondition>>,
    /// The operator to apply to the collection of conditions
    /// Possible values are: `OR`, `AND`.
    #[builder(into)]
    #[serde(rename = "logicalOperator")]
    pub r#logical_operator: Option<String>,
}
