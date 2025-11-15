#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountQueueProperties {
    /// A `cors_rule` block as defined above.
    #[builder(into)]
    #[serde(rename = "corsRules")]
    pub r#cors_rules: Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>,
    /// A `hour_metrics` block as defined below.
    #[builder(into)]
    #[serde(rename = "hourMetrics")]
    pub r#hour_metrics: Option<Box<super::super::types::storage::AccountQueuePropertiesHourMetrics>>,
    /// A `logging` block as defined below.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::super::types::storage::AccountQueuePropertiesLogging>>,
    /// A `minute_metrics` block as defined below.
    #[builder(into)]
    #[serde(rename = "minuteMetrics")]
    pub r#minute_metrics: Option<Box<super::super::types::storage::AccountQueuePropertiesMinuteMetrics>>,
}
