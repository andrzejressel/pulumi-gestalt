#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchStateHistory {
    /// (Output)
    /// The state of the batch at this point in history. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State).
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// (Output)
    /// Details about the state at this point in history.
    #[builder(into)]
    #[serde(rename = "stateMessage")]
    pub r#state_message: Option<String>,
    /// (Output)
    /// The time when the batch entered the historical state.
    #[builder(into)]
    #[serde(rename = "stateStartTime")]
    pub r#state_start_time: Option<String>,
}
