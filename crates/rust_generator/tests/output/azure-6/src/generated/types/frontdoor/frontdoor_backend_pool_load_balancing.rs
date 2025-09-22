#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorBackendPoolLoadBalancing {
    /// The additional latency in milliseconds for probes to fall into the lowest latency bucket. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "additionalLatencyMilliseconds")]
    pub r#additional_latency_milliseconds: Option<i32>,
    /// The ID of the FrontDoor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the name of the Load Balancer.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The number of samples to consider for load balancing decisions. Defaults to `4`.
    #[builder(into)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: Option<i32>,
    /// The number of samples within the sample period that must succeed. Defaults to `2`.
    #[builder(into)]
    #[serde(rename = "successfulSamplesRequired")]
    pub r#successful_samples_required: Option<i32>,
}
