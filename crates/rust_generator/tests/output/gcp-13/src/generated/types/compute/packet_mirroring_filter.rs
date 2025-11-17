#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PacketMirroringFilter {
    /// IP CIDR ranges that apply as a filter on the source (ingress) or
    /// destination (egress) IP in the IP header. Only IPv4 is supported.
    #[builder(into)]
    #[serde(rename = "cidrRanges")]
    pub r#cidr_ranges: Option<Vec<String>>,
    /// Direction of traffic to mirror.
    /// Default value is `BOTH`.
    /// Possible values are: `INGRESS`, `EGRESS`, `BOTH`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    /// Possible IP protocols including tcp, udp, icmp and esp
    #[builder(into)]
    #[serde(rename = "ipProtocols")]
    pub r#ip_protocols: Option<Vec<String>>,
}
