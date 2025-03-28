/// Manages a virtual network peering which allows resources to access other
/// resources in the linked virtual network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: peeredvnets-rg
///       location: West Europe
///   example-1:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: peternetwork1
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.1.0/24
///       location: ${example.location}
///   example-2:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: peternetwork2
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.2.0/24
///       location: ${example.location}
///   example-1VirtualNetworkPeering:
///     type: azure:network:VirtualNetworkPeering
///     name: example-1
///     properties:
///       name: peer1to2
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${["example-1"].name}
///       remoteVirtualNetworkId: ${["example-2"].id}
///   example-2VirtualNetworkPeering:
///     type: azure:network:VirtualNetworkPeering
///     name: example-2
///     properties:
///       name: peer2to1
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${["example-2"].name}
///       remoteVirtualNetworkId: ${["example-1"].id}
/// ```
///
///
/// ### Triggers)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: peeredvnets-rg
///       location: West Europe
///   example-1:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: peternetwork1
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.1.0/24
///       location: ${example.location}
///   example-2:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: peternetwork2
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.2.0/24
///       location: ${example.location}
///   example-1VirtualNetworkPeering:
///     type: azure:network:VirtualNetworkPeering
///     name: example-1
///     properties:
///       name: peer1to2
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${["example-1"].name}
///       remoteVirtualNetworkId: ${["example-2"].id}
///       triggers:
///         remote_address_space:
///           fn::invoke:
///             function: std:join
///             arguments:
///               separator: ','
///               input: ${["example-2"].addressSpaces}
///             return: result
///   example-2VirtualNetworkPeering:
///     type: azure:network:VirtualNetworkPeering
///     name: example-2
///     properties:
///       name: peer2to1
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${["example-2"].name}
///       remoteVirtualNetworkId: ${["example-1"].id}
///       triggers:
///         remote_address_space:
///           fn::invoke:
///             function: std:join
///             arguments:
///               separator: ','
///               input: ${["example-1"].addressSpaces}
///             return: result
/// ```
///
/// ## Note
///
/// Virtual Network peerings cannot be created, updated or deleted concurrently.
///
/// ## Import
///
/// Virtual Network Peerings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetworkPeering:VirtualNetworkPeering examplePeering /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1/virtualNetworkPeerings/myvnet1peering
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkPeeringArgs {
        /// Controls if forwarded traffic from VMs in the remote virtual network is allowed. Defaults to `false`.
        #[builder(into, default)]
        pub allow_forwarded_traffic: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Controls gatewayLinks can be used in the remote virtual network’s link to the local virtual network. Defaults to `false`.
        #[builder(into, default)]
        pub allow_gateway_transit: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Controls if the traffic from the local virtual network can reach the remote virtual network. Defaults to `true`.
        #[builder(into, default)]
        pub allow_virtual_network_access: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A list of local Subnet names that are Subnet peered with remote Virtual Network.
        #[builder(into, default)]
        pub local_subnet_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the virtual network peering. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether only IPv6 address space is peered for Subnet peering. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub only_ipv6_peering_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether complete Virtual Network address space is peered. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub peer_complete_virtual_networks_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A list of remote Subnet names from remote Virtual Network that are Subnet peered.
        #[builder(into, default)]
        pub remote_subnet_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The full Azure resource ID of the remote virtual network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub remote_virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the virtual network peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of key values pairs that can be used to sync network routes from the remote virtual network to the local virtual network. See the trigger example for an example on how to set it up.
        #[builder(into, default)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Controls if remote gateways can be used on the local virtual network. If the flag is set to `true`, and `allow_gateway_transit` on the remote peering is also `true`, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to `true`. This flag cannot be set if virtual network already has a gateway. Defaults to `false`.
        ///
        /// > **NOTE:** `use_remote_gateways` must be set to `false` if using Global Virtual Network Peerings.
        #[builder(into, default)]
        pub use_remote_gateways: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the virtual network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkPeeringResult {
        /// Controls if forwarded traffic from VMs in the remote virtual network is allowed. Defaults to `false`.
        pub allow_forwarded_traffic: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Controls gatewayLinks can be used in the remote virtual network’s link to the local virtual network. Defaults to `false`.
        pub allow_gateway_transit: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Controls if the traffic from the local virtual network can reach the remote virtual network. Defaults to `true`.
        pub allow_virtual_network_access: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of local Subnet names that are Subnet peered with remote Virtual Network.
        pub local_subnet_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the virtual network peering. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether only IPv6 address space is peered for Subnet peering. Changing this forces a new resource to be created.
        pub only_ipv6_peering_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether complete Virtual Network address space is peered. Defaults to `true`. Changing this forces a new resource to be created.
        pub peer_complete_virtual_networks_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A list of remote Subnet names from remote Virtual Network that are Subnet peered.
        pub remote_subnet_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The full Azure resource ID of the remote virtual network. Changing this forces a new resource to be created.
        pub remote_virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the virtual network peering. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of key values pairs that can be used to sync network routes from the remote virtual network to the local virtual network. See the trigger example for an example on how to set it up.
        pub triggers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Controls if remote gateways can be used on the local virtual network. If the flag is set to `true`, and `allow_gateway_transit` on the remote peering is also `true`, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to `true`. This flag cannot be set if virtual network already has a gateway. Defaults to `false`.
        ///
        /// > **NOTE:** `use_remote_gateways` must be set to `false` if using Global Virtual Network Peerings.
        pub use_remote_gateways: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the virtual network. Changing this forces a new resource to be created.
        pub virtual_network_name: pulumi_gestalt_rust::Output<String>,
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
        let local_subnet_names_binding = args.local_subnet_names.get_output(context);
        let name_binding = args.name.get_output(context);
        let only_ipv6_peering_enabled_binding = args
            .only_ipv6_peering_enabled
            .get_output(context);
        let peer_complete_virtual_networks_enabled_binding = args
            .peer_complete_virtual_networks_enabled
            .get_output(context);
        let remote_subnet_names_binding = args.remote_subnet_names.get_output(context);
        let remote_virtual_network_id_binding = args
            .remote_virtual_network_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let use_remote_gateways_binding = args.use_remote_gateways.get_output(context);
        let virtual_network_name_binding = args.virtual_network_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkPeering:VirtualNetworkPeering".into(),
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
                    name: "localSubnetNames".into(),
                    value: &local_subnet_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onlyIpv6PeeringEnabled".into(),
                    value: &only_ipv6_peering_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerCompleteVirtualNetworksEnabled".into(),
                    value: &peer_complete_virtual_networks_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteSubnetNames".into(),
                    value: &remote_subnet_names_binding.drop_type(),
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
                    name: "triggers".into(),
                    value: &triggers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useRemoteGateways".into(),
                    value: &use_remote_gateways_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkName".into(),
                    value: &virtual_network_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkPeeringResult {
            allow_forwarded_traffic: o.get_field("allowForwardedTraffic"),
            allow_gateway_transit: o.get_field("allowGatewayTransit"),
            allow_virtual_network_access: o.get_field("allowVirtualNetworkAccess"),
            local_subnet_names: o.get_field("localSubnetNames"),
            name: o.get_field("name"),
            only_ipv6_peering_enabled: o.get_field("onlyIpv6PeeringEnabled"),
            peer_complete_virtual_networks_enabled: o
                .get_field("peerCompleteVirtualNetworksEnabled"),
            remote_subnet_names: o.get_field("remoteSubnetNames"),
            remote_virtual_network_id: o.get_field("remoteVirtualNetworkId"),
            resource_group_name: o.get_field("resourceGroupName"),
            triggers: o.get_field("triggers"),
            use_remote_gateways: o.get_field("useRemoteGateways"),
            virtual_network_name: o.get_field("virtualNetworkName"),
        }
    }
}
