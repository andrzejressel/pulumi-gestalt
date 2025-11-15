#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkSubnet {
    /// The address prefixes to use for the subnet.
    #[builder(into)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Vec<String>,
    /// Enable default outbound access to the internet for the subnet. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "defaultOutboundAccessEnabled")]
    pub r#default_outbound_access_enabled: Option<bool>,
    /// One or more `delegation` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "delegation")]
    pub r#delegation: Option<Box<super::super::types::network::VirtualNetworkSubnetDelegation>>,
    /// The ID of this subnet.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the subnet.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Enable or Disable network policies for the private endpoint on the subnet. Possible values are `Disabled`, `Enabled`, `NetworkSecurityGroupEnabled` and `RouteTableEnabled`. Defaults to `Disabled`.
    /// 
    /// > **NOTE:** If you don't want to use network policies like user-defined Routes and Network Security Groups, you need to set `private_endpoint_network_policies` in the subnet to `Disabled`. This setting only applies to Private Endpoints in the Subnet and affects all Private Endpoints in the Subnet.
    /// 
    /// > **NOTE:** If you want to use network policies like user-defined Routes and Network Security Groups, you need to set the `private_endpoint_network_policies` in the Subnet to `Enabled`/`NetworkSecurityGroupEnabled`/`RouteTableEnabled`. This setting only applies to Private Endpoints in the Subnet and affects all Private Endpoints in the Subnet.
    /// 
    /// > **NOTE:** See more details from [Manage network policies for Private Endpoints](https://learn.microsoft.com/en-gb/azure/private-link/disable-private-endpoint-network-policy?tabs=network-policy-portal).
    #[builder(into)]
    #[serde(rename = "privateEndpointNetworkPolicies")]
    pub r#private_endpoint_network_policies: Option<String>,
    /// Enable or Disable network policies for the private link service on the subnet. Defaults to `true`.
    /// 
    /// > **NOTE:** When configuring Azure Private Link service, the explicit setting `private_link_service_network_policies_enabled` must be set to `false` in the subnet since Private Link Service does not support network policies like user-defined Routes and Network Security Groups. This setting only affects the Private Link service. For other resources in the subnet, access is controlled based on the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource. See more details from [Manage network policies for Private Link Services](https://learn.microsoft.com/en-gb/azure/private-link/disable-private-link-service-network-policy?tabs=private-link-network-policy-powershell).
    #[builder(into)]
    #[serde(rename = "privateLinkServiceNetworkPoliciesEnabled")]
    pub r#private_link_service_network_policies_enabled: Option<bool>,
    /// The ID of the Route Table that should be associated with this subnet.
    #[builder(into)]
    #[serde(rename = "routeTableId")]
    pub r#route_table_id: Option<String>,
    /// The Network Security Group to associate with the subnet. (Referenced by `id`, ie. `azurerm_network_security_group.example.id`)
    #[builder(into)]
    #[serde(rename = "securityGroup")]
    pub r#security_group: Option<String>,
    /// The list of IDs of Service Endpoint Policies to associate with the subnet.
    #[builder(into)]
    #[serde(rename = "serviceEndpointPolicyIds")]
    pub r#service_endpoint_policy_ids: Option<Vec<String>>,
    /// The list of Service endpoints to associate with the subnet. Possible values include: `Microsoft.AzureActiveDirectory`, `Microsoft.AzureCosmosDB`, `Microsoft.ContainerRegistry`, `Microsoft.EventHub`, `Microsoft.KeyVault`, `Microsoft.ServiceBus`, `Microsoft.Sql`, `Microsoft.Storage`, `Microsoft.Storage.Global` and `Microsoft.Web`.
    #[builder(into)]
    #[serde(rename = "serviceEndpoints")]
    pub r#service_endpoints: Option<Vec<String>>,
}
