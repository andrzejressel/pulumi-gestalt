#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkAttachedDataNetworkNetworkAddressPortTranslation {
    /// Pinhole timeout for ICMP pinholes in seconds. Must between `1` to `180`, Default to `180`.
    #[builder(into)]
    #[serde(rename = "icmpPinholeTimeoutInSeconds")]
    pub r#icmp_pinhole_timeout_in_seconds: Option<i32>,
    /// Maximum number of UDP and TCP pinholes that can be open simultaneously on the core interface. For 5G networks, this is the N6 interface. For 4G networks, this is the SGi interface. Must be between 1 and 65536.
    #[builder(into)]
    #[serde(rename = "pinholeMaximumNumber")]
    pub r#pinhole_maximum_number: Option<i32>,
    /// A `port_range` block as defined below.
    #[builder(into)]
    #[serde(rename = "portRange")]
    pub r#port_range: Option<Box<super::super::types::mobile::NetworkAttachedDataNetworkNetworkAddressPortTranslationPortRange>>,
    /// Pinhole timeout for TCP pinholes in seconds. Must between `1` to `180`, Default to `180`.
    #[builder(into)]
    #[serde(rename = "tcpPinholeTimeoutInSeconds")]
    pub r#tcp_pinhole_timeout_in_seconds: Option<i32>,
    /// Minimum time in seconds that will pass before a TCP port that was used by a closed pinhole can be reused. Defaults to `120`.
    #[builder(into)]
    #[serde(rename = "tcpPortReuseMinimumHoldTimeInSeconds")]
    pub r#tcp_port_reuse_minimum_hold_time_in_seconds: Option<i32>,
    /// Pinhole timeout for UDP pinholes in seconds. Must between `1` to `180`, Default to `180`.
    #[builder(into)]
    #[serde(rename = "udpPinholeTimeoutInSeconds")]
    pub r#udp_pinhole_timeout_in_seconds: Option<i32>,
    /// Minimum time in seconds that will pass before a UDP port that was used by a closed pinhole can be reused. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "udpPortReuseMinimumHoldTimeInSeconds")]
    pub r#udp_port_reuse_minimum_hold_time_in_seconds: Option<i32>,
}
