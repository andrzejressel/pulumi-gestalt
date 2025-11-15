#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceNetworkInterface {
    /// Access configurations, i.e. IPs via which this
    /// instance can be accessed via the Internet. Structure documented below.
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Vec<super::super::types::compute::GetInstanceNetworkInterfaceAccessConfig>,
    /// An array of alias IP ranges for this network interface. Structure documented below.
    #[builder(into)]
    #[serde(rename = "aliasIpRanges")]
    pub r#alias_ip_ranges: Vec<super::super::types::compute::GetInstanceNetworkInterfaceAliasIpRange>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into)]
    #[serde(rename = "internalIpv6PrefixLength")]
    pub r#internal_ipv_6_prefix_length: i32,
    /// An array of IPv6 access configurations for this interface. Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig specified, then this instance will have no external IPv6 Internet access.
    #[builder(into)]
    #[serde(rename = "ipv6AccessConfigs")]
    pub r#ipv_6_access_configs: Vec<super::super::types::compute::GetInstanceNetworkInterfaceIpv6AccessConfig>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6AccessType")]
    pub r#ipv_6_access_type: String,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: String,
    /// The name of the instance. One of `name` or `self_link` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name or self_link of the network attached to this interface.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// Beta The URL of the network attachment to this interface.
    #[builder(into)]
    #[serde(rename = "networkAttachment")]
    pub r#network_attachment: String,
    /// The internal ip address of the instance, either manually or dynamically assigned.
    #[builder(into)]
    #[serde(rename = "networkIp")]
    pub r#network_ip: String,
    /// The type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, IDPF, MRDMA, and IRDMA
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: String,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: i32,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: String,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used.
    #[builder(into)]
    #[serde(rename = "stackType")]
    pub r#stack_type: String,
    /// The name or self_link of the subnetwork attached to this interface.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: String,
    /// The project in which the subnetwork belongs.
    #[builder(into)]
    #[serde(rename = "subnetworkProject")]
    pub r#subnetwork_project: String,
}
