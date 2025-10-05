/// Manages an Express Route Circuit Connection.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let example2 = express_route_port::create(
///         "example2",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("example-erport2")
///             .peering_location("Allied-Toronto-King-West")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let example2ExpressRouteCircuit = express_route_circuit::create(
///         "example2ExpressRouteCircuit",
///         ExpressRouteCircuitArgs::builder()
///             .bandwidth_in_gbps(5)
///             .express_route_port_id("${example2.id}")
///             .location("${example.location}")
///             .name("example-ercircuit2")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ExpressRouteCircuitSku::builder()
///                     .family("MeteredData")
///                     .tier("Standard")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let example2ExpressRouteCircuitPeering = express_route_circuit_peering::create(
///         "example2ExpressRouteCircuitPeering",
///         ExpressRouteCircuitPeeringArgs::builder()
///             .express_route_circuit_name("${example2ExpressRouteCircuit.name}")
///             .peer_asn(100)
///             .peering_type("AzurePrivatePeering")
///             .primary_peer_address_prefix("192.168.1.0/30")
///             .resource_group_name("${example.name}")
///             .secondary_peer_address_prefix("192.168.1.0/30")
///             .shared_key("ItsASecret")
///             .vlan_id(100)
///             .build_struct(),
///     );
///     let exampleExpressRouteCircuit = express_route_circuit::create(
///         "exampleExpressRouteCircuit",
///         ExpressRouteCircuitArgs::builder()
///             .bandwidth_in_gbps(5)
///             .express_route_port_id("${exampleExpressRoutePort.id}")
///             .location("${example.location}")
///             .name("example-ercircuit")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ExpressRouteCircuitSku::builder()
///                     .family("MeteredData")
///                     .tier("Standard")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleExpressRouteCircuitConnection = express_route_circuit_connection::create(
///         "exampleExpressRouteCircuitConnection",
///         ExpressRouteCircuitConnectionArgs::builder()
///             .address_prefix_ipv_4("192.169.9.0/29")
///             .authorization_key("846a1918-b7a2-4917-b43c-8c4cdaee006a")
///             .name("example-ercircuitconnection")
///             .peer_peering_id("${example2ExpressRouteCircuitPeering.id}")
///             .peering_id("${exampleExpressRouteCircuitPeering.id}")
///             .build_struct(),
///     );
///     let exampleExpressRouteCircuitPeering = express_route_circuit_peering::create(
///         "exampleExpressRouteCircuitPeering",
///         ExpressRouteCircuitPeeringArgs::builder()
///             .express_route_circuit_name("${exampleExpressRouteCircuit.name}")
///             .peer_asn(100)
///             .peering_type("AzurePrivatePeering")
///             .primary_peer_address_prefix("192.168.1.0/30")
///             .resource_group_name("${example.name}")
///             .secondary_peer_address_prefix("192.168.1.0/30")
///             .shared_key("ItsASecret")
///             .vlan_id(100)
///             .build_struct(),
///     );
///     let exampleExpressRoutePort = express_route_port::create(
///         "exampleExpressRoutePort",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("example-erport")
///             .peering_location("Equinix-Seattle-SE2")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Express Route Circuit Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRouteCircuitConnection:ExpressRouteCircuitConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/expressRouteCircuits/circuit1/peerings/peering1/connections/connection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod express_route_circuit_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitConnectionArgs {
        /// The IPv4 address space from which to allocate customer address for global reach. Changing this forces a new Express Route Circuit Connection to be created.
        #[builder(into)]
        pub address_prefix_ipv4: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv6 address space from which to allocate customer addresses for global reach.
        ///
        /// > **NOTE:** `address_prefix_ipv6` cannot be set when ExpressRoute Circuit Connection with ExpressRoute Circuit based on ExpressRoute Port.
        #[builder(into, default)]
        pub address_prefix_ipv6: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The authorization key which is associated with the Express Route Circuit Connection.
        #[builder(into, default)]
        pub authorization_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Express Route Circuit Connection. Changing this forces a new Express Route Circuit Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the peered Express Route Circuit Private Peering. Changing this forces a new Express Route Circuit Connection to be created.
        #[builder(into)]
        pub peer_peering_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Express Route Circuit Private Peering that this Express Route Circuit Connection connects with. Changing this forces a new Express Route Circuit Connection to be created.
        #[builder(into)]
        pub peering_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitConnectionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 address space from which to allocate customer address for global reach. Changing this forces a new Express Route Circuit Connection to be created.
        pub address_prefix_ipv4: pulumi_gestalt_rust::Output<String>,
        /// The IPv6 address space from which to allocate customer addresses for global reach.
        ///
        /// > **NOTE:** `address_prefix_ipv6` cannot be set when ExpressRoute Circuit Connection with ExpressRoute Circuit based on ExpressRoute Port.
        pub address_prefix_ipv6: pulumi_gestalt_rust::Output<Option<String>>,
        /// The authorization key which is associated with the Express Route Circuit Connection.
        pub authorization_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Express Route Circuit Connection. Changing this forces a new Express Route Circuit Connection to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the peered Express Route Circuit Private Peering. Changing this forces a new Express Route Circuit Connection to be created.
        pub peer_peering_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Express Route Circuit Private Peering that this Express Route Circuit Connection connects with. Changing this forces a new Express Route Circuit Connection to be created.
        pub peering_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExpressRouteCircuitConnectionArgs,
    ) -> ExpressRouteCircuitConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_prefix_ipv4_binding = args.address_prefix_ipv4.get_output(context);
        let address_prefix_ipv6_binding = args.address_prefix_ipv6.get_output(context);
        let authorization_key_binding = args.authorization_key.get_output(context);
        let name_binding = args.name.get_output(context);
        let peer_peering_id_binding = args.peer_peering_id.get_output(context);
        let peering_id_binding = args.peering_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuitConnection:ExpressRouteCircuitConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressPrefixIpv4".into(),
                    value: &address_prefix_ipv4_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressPrefixIpv6".into(),
                    value: &address_prefix_ipv6_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationKey".into(),
                    value: &authorization_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerPeeringId".into(),
                    value: &peer_peering_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeringId".into(),
                    value: &peering_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExpressRouteCircuitConnectionResult {
            id: o.get_field("id"),
            address_prefix_ipv4: o.get_field("addressPrefixIpv4"),
            address_prefix_ipv6: o.get_field("addressPrefixIpv6"),
            authorization_key: o.get_field("authorizationKey"),
            name: o.get_field("name"),
            peer_peering_id: o.get_field("peerPeeringId"),
            peering_id: o.get_field("peeringId"),
        }
    }
}
