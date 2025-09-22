#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VmwareReplicatedVmNetworkInterface {
    /// Whether this `network_interface` is primary for the replicated VM.
    #[builder(into)]
    #[serde(rename = "isPrimary")]
    pub r#is_primary: bool,
    /// Mac address of the network interface of source VM.
    #[builder(into)]
    #[serde(rename = "sourceMacAddress")]
    pub r#source_mac_address: String,
    /// Static IP to assign when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetStaticIp")]
    pub r#target_static_ip: Option<String>,
    /// Name of the subnet to use when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetSubnetName")]
    pub r#target_subnet_name: Option<String>,
    /// Name of the subnet to use when a test failover is done.
    #[builder(into)]
    #[serde(rename = "testSubnetName")]
    pub r#test_subnet_name: Option<String>,
}
