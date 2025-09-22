#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFrontdoorOriginGroupLoadBalancing {
    /// Specifies the additional latency in milliseconds for probes to fall into the lowest latency bucket.
    #[builder(into)]
    #[serde(rename = "additionalLatencyInMilliseconds")]
    pub r#additional_latency_in_milliseconds: i32,
    /// Specifies the number of samples to consider for load balancing decisions.
    #[builder(into)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: i32,
    /// Specifies the number of samples within the sample period that must succeed.
    #[builder(into)]
    #[serde(rename = "successfulSamplesRequired")]
    pub r#successful_samples_required: i32,
}
