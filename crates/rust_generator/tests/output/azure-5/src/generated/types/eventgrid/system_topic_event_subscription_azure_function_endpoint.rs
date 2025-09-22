#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SystemTopicEventSubscriptionAzureFunctionEndpoint {
    /// Specifies the ID of the Function where the Event Subscription will receive events. This must be the functions ID in format {function_app.id}/functions/{name}.
    #[builder(into)]
    #[serde(rename = "functionId")]
    pub r#function_id: String,
    /// Maximum number of events per batch.
    #[builder(into)]
    #[serde(rename = "maxEventsPerBatch")]
    pub r#max_events_per_batch: Option<i32>,
    /// Preferred batch size in Kilobytes.
    #[builder(into)]
    #[serde(rename = "preferredBatchSizeInKilobytes")]
    pub r#preferred_batch_size_in_kilobytes: Option<i32>,
}
