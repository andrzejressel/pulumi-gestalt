#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2VmNetworkConfig {
    /// Allows the TPU node to send and receive packets with non-matching destination or source
    /// IPs. This is required if you plan to use the TPU workers to forward routes.
    #[builder(into)]
    #[serde(rename = "canIpForward")]
    pub r#can_ip_forward: Option<bool>,
    /// Indicates that external IP addresses would be associated with the TPU workers. If set to
    /// false, the specified subnetwork or network should have Private Google Access enabled.
    #[builder(into)]
    #[serde(rename = "enableExternalIps")]
    pub r#enable_external_ips: Option<bool>,
    /// The name of the network for the TPU node. It must be a preexisting Google Compute Engine
    /// network. If none is provided, "default" will be used.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Specifies networking queue count for TPU VM instance's network interface.
    #[builder(into)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: Option<i32>,
    /// The name of the subnetwork for the TPU node. It must be a preexisting Google Compute
    /// Engine subnetwork. If none is provided, "default" will be used.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
}
