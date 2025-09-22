#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressionsCondition {
    /// The minimum data risk score that triggers the condition.
    /// Possible values are: `HIGH`, `MEDIUM_OR_HIGH`.
    #[builder(into)]
    #[serde(rename = "minimumRiskScore")]
    pub r#minimum_risk_score: Option<String>,
    /// The minimum sensitivity level that triggers the condition.
    /// Possible values are: `HIGH`, `MEDIUM_OR_HIGH`.
    #[builder(into)]
    #[serde(rename = "minimumSensitivityScore")]
    pub r#minimum_sensitivity_score: Option<String>,
}
