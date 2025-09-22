#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolNetworkConfiguration {
    /// Whether to enable accelerated networking. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "acceleratedNetworkingEnabled")]
    pub r#accelerated_networking_enabled: Option<bool>,
    /// The scope of dynamic vnet assignment. Allowed values: `none`, `job`. Changing this forces a new resource to be created. Defaults to `none`.
    #[builder(into)]
    #[serde(rename = "dynamicVnetAssignmentScope")]
    pub r#dynamic_vnet_assignment_scope: Option<String>,
    /// A list of `endpoint_configuration` blocks that can be used to address specific ports on an individual compute node externally as defined below. Set as documented in the inbound_nat_pools block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "endpointConfigurations")]
    pub r#endpoint_configurations: Option<Vec<super::super::types::batch::PoolNetworkConfigurationEndpointConfiguration>>,
    /// Type of public IP address provisioning. Supported values are `BatchManaged`, `UserManaged` and `NoPublicIPAddresses`.
    #[builder(into)]
    #[serde(rename = "publicAddressProvisioningType")]
    pub r#public_address_provisioning_type: Option<String>,
    /// A list of public IP ids that will be allocated to nodes. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publicIps")]
    pub r#public_ips: Option<Vec<String>>,
    /// The ARM resource identifier of the virtual network subnet which the compute nodes of the pool will join. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
