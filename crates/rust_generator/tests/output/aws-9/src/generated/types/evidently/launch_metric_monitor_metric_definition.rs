#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchMetricMonitorMetricDefinition {
    /// Specifies the entity, such as a user or session, that does an action that causes a metric value to be recorded. An example is `userDetails.userID`.
    #[builder(into)]
    #[serde(rename = "entityIdKey")]
    pub r#entity_id_key: String,
    /// Specifies The EventBridge event pattern that defines how the metric is recorded.
    #[builder(into)]
    #[serde(rename = "eventPattern")]
    pub r#event_pattern: Option<String>,
    /// Specifies the name for the metric.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies a label for the units that the metric is measuring.
    #[builder(into)]
    #[serde(rename = "unitLabel")]
    pub r#unit_label: Option<String>,
    /// Specifies the value that is tracked to produce the metric.
    #[builder(into)]
    #[serde(rename = "valueKey")]
    pub r#value_key: String,
}
