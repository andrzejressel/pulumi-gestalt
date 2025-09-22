#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IpSetIpSetDescriptor {
    /// Type of the IP address - `IPV4` or `IPV6`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// An IPv4 or IPv6 address specified via CIDR notationE.g., `192.0.2.44/32` or `1111:0000:0000:0000:0000:0000:0000:0000/64`
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
