/// Manages a Databricks Virtual Network Peering
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   remote:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: remote-vnet
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.1.0/24
///       location: ${example.location}
///   exampleWorkspace:
///     type: azure:databricks:Workspace
///     name: example
///     properties:
///       name: example-workspace
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: standard
///   exampleVirtualNetworkPeering:
///     type: azure:databricks:VirtualNetworkPeering
///     name: example
///     properties:
///       name: databricks-vnet-peer
///       resourceGroupName: ${example.name}
///       workspaceId: ${exampleWorkspace.id}
///       remoteAddressSpacePrefixes: ${remote.addressSpaces}
///       remoteVirtualNetworkId: ${remote.id}
///       allowVirtualNetworkAccess: true
///   remoteVirtualNetworkPeering:
///     type: azure:network:VirtualNetworkPeering
///     name: remote
///     properties:
///       name: peer-to-databricks
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${remote.name}
///       remoteVirtualNetworkId: ${exampleVirtualNetworkPeering.virtualNetworkId}
///       allowVirtualNetworkAccess: true
/// ```
///
/// ## Import
///
/// Databrick Virtual Network Peerings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:databricks/virtualNetworkPeering:VirtualNetworkPeering example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Databricks/workspaces/workspace1/virtualNetworkPeerings/peering1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkPeeringArgs {
        /// Can the forwarded traffic from the VMs in the local virtual network be forwarded to the remote virtual network? Defaults to `false`.
        #[builder(into, default)]
        pub allow_forwarded_traffic: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Can the gateway links be used in the remote virtual network to link to the Databricks virtual network? Defaults to `false`.
        #[builder(into, default)]
        pub allow_gateway_transit: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Can the VMs in the local virtual network space access the VMs in the remote virtual network space? Defaults to `true`.
        #[builder(into, default)]
        pub allow_virtual_network_access: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name of the Databricks Virtual Network Peering resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of address blocks reserved for the remote virtual network in CIDR notation. Changing this forces a new resource to be created.
        #[builder(into)]
        pub remote_address_space_prefixes: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// The ID of the remote virtual network. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The remote virtual network should be in the same region as the databricks workspace. Please see the [product documentation](https://learn.microsoft.com/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering) for more information.
        #[builder(into)]
        pub remote_virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Databricks Virtual Network Peering should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Can remote gateways be used on the Databricks virtual network? Defaults to `false`.
        ///
        /// > **NOTE:** If the `use_remote_gateways` is set to `true`, and `allow_gateway_transit` on the remote peering is also `true`, the virtual network will use the gateways of the remote virtual network for transit. Only one peering can have this flag set to `true`. `use_remote_gateways` cannot be set if the virtual network already has a gateway.
        #[builder(into, default)]
        pub use_remote_gateways: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Databricks Workspace that this Databricks Virtual Network Peering is bound. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkPeeringResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of address blocks reserved for this virtual network in CIDR notation.
        pub address_space_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Can the forwarded traffic from the VMs in the local virtual network be forwarded to the remote virtual network? Defaults to `false`.
        pub allow_forwarded_traffic: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Can the gateway links be used in the remote virtual network to link to the Databricks virtual network? Defaults to `false`.
        pub allow_gateway_transit: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Can the VMs in the local virtual network space access the VMs in the remote virtual network space? Defaults to `true`.
        pub allow_virtual_network_access: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Databricks Virtual Network Peering resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of address blocks reserved for the remote virtual network in CIDR notation. Changing this forces a new resource to be created.
        pub remote_address_space_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the remote virtual network. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The remote virtual network should be in the same region as the databricks workspace. Please see the [product documentation](https://learn.microsoft.com/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering) for more information.
        pub remote_virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the Databricks Virtual Network Peering should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Can remote gateways be used on the Databricks virtual network? Defaults to `false`.
        ///
        /// > **NOTE:** If the `use_remote_gateways` is set to `true`, and `allow_gateway_transit` on the remote peering is also `true`, the virtual network will use the gateways of the remote virtual network for transit. Only one peering can have this flag set to `true`. `use_remote_gateways` cannot be set if the virtual network already has a gateway.
        pub use_remote_gateways: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the internal Virtual Network used by the DataBricks Workspace.
        ///
        /// > **NOTE:** The `virtual_network_id` field is the value you must supply to the `azure.network.VirtualNetworkPeering` resources `remote_virtual_network_id` field to successfully peer the Databricks Virtual Network with the remote virtual network.
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Databricks Workspace that this Databricks Virtual Network Peering is bound. Changing this forces a new resource to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkPeeringArgs,
    ) -> VirtualNetworkPeeringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_forwarded_traffic_binding = args
            .allow_forwarded_traffic
            .get_output(context);
        let allow_gateway_transit_binding = args
            .allow_gateway_transit
            .get_output(context);
        let allow_virtual_network_access_binding = args
            .allow_virtual_network_access
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let remote_address_space_prefixes_binding = args
            .remote_address_space_prefixes
            .get_output(context);
        let remote_virtual_network_id_binding = args
            .remote_virtual_network_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let use_remote_gateways_binding = args.use_remote_gateways.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:databricks/virtualNetworkPeering:VirtualNetworkPeering".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowForwardedTraffic".into(),
                    value: &allow_forwarded_traffic_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowGatewayTransit".into(),
                    value: &allow_gateway_transit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowVirtualNetworkAccess".into(),
                    value: &allow_virtual_network_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteAddressSpacePrefixes".into(),
                    value: &remote_address_space_prefixes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteVirtualNetworkId".into(),
                    value: &remote_virtual_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useRemoteGateways".into(),
                    value: &use_remote_gateways_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkPeeringResult {
            id: o.get_field("id"),
            address_space_prefixes: o.get_field("addressSpacePrefixes"),
            allow_forwarded_traffic: o.get_field("allowForwardedTraffic"),
            allow_gateway_transit: o.get_field("allowGatewayTransit"),
            allow_virtual_network_access: o.get_field("allowVirtualNetworkAccess"),
            name: o.get_field("name"),
            remote_address_space_prefixes: o.get_field("remoteAddressSpacePrefixes"),
            remote_virtual_network_id: o.get_field("remoteVirtualNetworkId"),
            resource_group_name: o.get_field("resourceGroupName"),
            use_remote_gateways: o.get_field("useRemoteGateways"),
            virtual_network_id: o.get_field("virtualNetworkId"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
