/// Manages the association between a Network Interface and a Load Balancer's NAT Rule.
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
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder().name("primary")
///                     .publicIpAddressId("${examplePublicIp.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-lb")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNatRule = nat_rule::create(
///         "exampleNatRule",
///         NatRuleArgs::builder()
///             .backend_port(3389)
///             .frontend_ip_configuration_name("primary")
///             .frontend_port(3389)
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("RDPAccess")
///             .protocol("Tcp")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("testconfiguration1")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkInterfaceNatRuleAssociation = network_interface_nat_rule_association::create(
///         "exampleNetworkInterfaceNatRuleAssociation",
///         NetworkInterfaceNatRuleAssociationArgs::builder()
///             .ip_configuration_name("testconfiguration1")
///             .nat_rule_id("${exampleNatRule.id}")
///             .network_interface_id("${exampleNetworkInterface.id}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("example-pip")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Associations between Network Interfaces and Load Balancer NAT Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkInterfaceNatRuleAssociation:NetworkInterfaceNatRuleAssociation association1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkInterfaces/nic1/ipConfigurations/example|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/inboundNatRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_interface_nat_rule_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceNatRuleAssociationArgs {
        /// The Name of the IP Configuration within the Network Interface which should be connected to the NAT Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ip_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Load Balancer NAT Rule which this Network Interface which should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub nat_rule_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceNatRuleAssociationResult {
        /// The Name of the IP Configuration within the Network Interface which should be connected to the NAT Rule. Changing this forces a new resource to be created.
        pub ip_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Load Balancer NAT Rule which this Network Interface which should be connected to. Changing this forces a new resource to be created.
        pub nat_rule_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkInterfaceNatRuleAssociationArgs,
    ) -> NetworkInterfaceNatRuleAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ip_configuration_name_binding = args
            .ip_configuration_name
            .get_output(context);
        let nat_rule_id_binding = args.nat_rule_id.get_output(context);
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkInterfaceNatRuleAssociation:NetworkInterfaceNatRuleAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipConfigurationName".into(),
                    value: &ip_configuration_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "natRuleId".into(),
                    value: &nat_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkInterfaceNatRuleAssociationResult {
            ip_configuration_name: o.get_field("ipConfigurationName"),
            nat_rule_id: o.get_field("natRuleId"),
            network_interface_id: o.get_field("networkInterfaceId"),
        }
    }
}
