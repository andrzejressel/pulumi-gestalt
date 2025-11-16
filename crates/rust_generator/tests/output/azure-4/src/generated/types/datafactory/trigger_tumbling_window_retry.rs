#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerTumblingWindowRetry {
    /// The maximum retry attempts if the pipeline run failed.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// The Interval in seconds between each retry if the pipeline run failed. Defaults to `30`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
}
