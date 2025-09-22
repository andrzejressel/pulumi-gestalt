#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertPolicyConditionConditionThresholdTrigger {
    /// The absolute number of time series
    /// that must fail the predicate for the
    /// condition to be triggered.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<i32>,
    /// The percentage of time series that
    /// must fail the predicate for the
    /// condition to be triggered.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: Option<f64>,
}
