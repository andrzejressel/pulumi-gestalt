#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkSimPolicySliceDataNetworkSessionAggregateMaximumBitRate {
    /// Downlink bit rate. Must be a number followed by `Kbps`, `Mbps`, `Gbps` or `Tbps`.
    #[builder(into)]
    #[serde(rename = "downlink")]
    pub r#downlink: String,
    /// Uplink bit rate. Must be a number followed by `Kbps`, `Mbps`, `Gbps` or `Tbps`.
    #[builder(into)]
    #[serde(rename = "uplink")]
    pub r#uplink: String,
}
