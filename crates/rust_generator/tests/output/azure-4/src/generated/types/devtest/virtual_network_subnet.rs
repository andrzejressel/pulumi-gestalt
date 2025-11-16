#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkSubnet {
    /// Specifies the name of the Dev Test Virtual Network. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A `shared_public_ip_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "sharedPublicIpAddress")]
    pub r#shared_public_ip_address: Option<Box<super::super::types::devtest::VirtualNetworkSubnetSharedPublicIpAddress>>,
    /// Can this subnet be used for creating Virtual Machines? Possible values are `Allow`, `Default` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "useInVirtualMachineCreation")]
    pub r#use_in_virtual_machine_creation: Option<String>,
    /// Can Virtual Machines in this Subnet use Public IP Addresses? Possible values are `Allow`, `Default` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "usePublicIpAddress")]
    pub r#use_public_ip_address: Option<String>,
}
