#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyBasedRouteFilter {
    /// The destination IP range of outgoing packets that this policy-based route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "destRange")]
    pub r#dest_range: Option<String>,
    /// The IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'.
    #[builder(into)]
    #[serde(rename = "ipProtocol")]
    pub r#ip_protocol: Option<String>,
    /// Internet protocol versions this policy-based route applies to.
    /// Possible values are: `IPV4`.
    #[builder(into)]
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: String,
    /// The source IP range of outgoing packets that this policy-based route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
    #[builder(into)]
    #[serde(rename = "srcRange")]
    pub r#src_range: Option<String>,
}
