#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BudgetActionActionThreshold {
    /// The type of threshold for a notification. Valid values are `PERCENTAGE` or `ABSOLUTE_VALUE`.
    #[builder(into)]
    #[serde(rename = "actionThresholdType")]
    pub r#action_threshold_type: String,
    /// The threshold of a notification.
    #[builder(into)]
    #[serde(rename = "actionThresholdValue")]
    pub r#action_threshold_value: f64,
}
