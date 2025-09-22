#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SystemTopicEventSubscriptionRetryPolicy {
    /// Specifies the time to live (in minutes) for events. Supported range is `1` to `1440`. See [official documentation](https://docs.microsoft.com/azure/event-grid/manage-event-delivery#set-retry-policy) for more details.
    #[builder(into)]
    #[serde(rename = "eventTimeToLive")]
    pub r#event_time_to_live: i32,
    /// Specifies the maximum number of delivery retry attempts for events.
    #[builder(into)]
    #[serde(rename = "maxDeliveryAttempts")]
    pub r#max_delivery_attempts: i32,
}
