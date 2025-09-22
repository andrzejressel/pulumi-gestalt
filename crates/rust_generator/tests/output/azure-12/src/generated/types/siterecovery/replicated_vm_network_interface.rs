#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicatedVmNetworkInterface {
    /// Id of the public IP object to use when a test failover is done.
    #[builder(into)]
    #[serde(rename = "failoverTestPublicIpAddressId")]
    pub r#failover_test_public_ip_address_id: Option<String>,
    /// Static IP to assign when a test failover is done.
    #[builder(into)]
    #[serde(rename = "failoverTestStaticIp")]
    pub r#failover_test_static_ip: Option<String>,
    /// Name of the subnet to to use when a test failover is done.
    #[builder(into)]
    #[serde(rename = "failoverTestSubnetName")]
    pub r#failover_test_subnet_name: Option<String>,
    /// Id of the public IP object to use when a failover is done.
    #[builder(into)]
    #[serde(rename = "recoveryPublicIpAddressId")]
    pub r#recovery_public_ip_address_id: Option<String>,
    /// (Required if the network_interface block is specified) Id source network interface.
    #[builder(into)]
    #[serde(rename = "sourceNetworkInterfaceId")]
    pub r#source_network_interface_id: Option<String>,
    /// Static IP to assign when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetStaticIp")]
    pub r#target_static_ip: Option<String>,
    /// Name of the subnet to to use when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetSubnetName")]
    pub r#target_subnet_name: Option<String>,
}
