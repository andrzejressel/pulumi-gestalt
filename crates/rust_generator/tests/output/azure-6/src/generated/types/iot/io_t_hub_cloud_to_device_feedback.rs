#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubCloudToDeviceFeedback {
    /// The lock duration for the feedback queue, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 5 and 300 seconds. Defaults to `PT60S`.
    #[builder(into)]
    #[serde(rename = "lockDuration")]
    pub r#lock_duration: Option<String>,
    /// The maximum delivery count for the feedback queue. This value must be between `1` and `100`. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "maxDeliveryCount")]
    pub r#max_delivery_count: Option<i32>,
    /// The retention time for service-bound feedback messages, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 1 minute and 48 hours. Defaults to `PT1H`.
    #[builder(into)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Option<String>,
}
