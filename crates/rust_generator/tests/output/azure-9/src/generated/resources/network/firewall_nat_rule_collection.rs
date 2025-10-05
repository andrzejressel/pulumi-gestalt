/// Manages a NAT Rule Collection within an Azure Firewall.
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: testvnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: AzureFirewallSubnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: testpip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       sku: Standard
///   exampleFirewall:
///     type: azure:network:Firewall
///     name: example
///     properties:
///       name: testfirewall
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: AZFW_VNet
///       skuTier: Standard
///       ipConfigurations:
///         - name: configuration
///           subnetId: ${exampleSubnet.id}
///           publicIpAddressId: ${examplePublicIp.id}
///   exampleFirewallNatRuleCollection:
///     type: azure:network:FirewallNatRuleCollection
///     name: example
///     properties:
///       name: testcollection
///       azureFirewallName: ${exampleFirewall.name}
///       resourceGroupName: ${example.name}
///       priority: 100
///       action: Dnat
///       rules:
///         - name: testrule
///           sourceAddresses:
///             - 10.0.0.0/16
///           destinationPorts:
///             - '53'
///           destinationAddresses:
///             - ${examplePublicIp.ipAddress}
///           translatedPort: 53
///           translatedAddress: 8.8.8.8
///           protocols:
///             - TCP
///             - UDP
/// ```
///
/// ## Import
///
/// Azure Firewall NAT Rule Collections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/firewallNatRuleCollection:FirewallNatRuleCollection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/azureFirewalls/myfirewall/natRuleCollections/mycollection
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_nat_rule_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallNatRuleCollectionArgs {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Dnat` and `Snat`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Firewall in which the NAT Rule Collection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub azure_firewall_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the NAT Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `rule` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::FirewallNatRuleCollectionRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallNatRuleCollectionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Dnat` and `Snat`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Firewall in which the NAT Rule Collection should be created. Changing this forces a new resource to be created.
        pub azure_firewall_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the NAT Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `rule` blocks as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::FirewallNatRuleCollectionRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallNatRuleCollectionArgs,
    ) -> FirewallNatRuleCollectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let azure_firewall_name_binding = args.azure_firewall_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/firewallNatRuleCollection:FirewallNatRuleCollection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: &action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureFirewallName".into(),
                    value: &azure_firewall_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallNatRuleCollectionResult {
            id: o.get_field("id"),
            action: o.get_field("action"),
            azure_firewall_name: o.get_field("azureFirewallName"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            resource_group_name: o.get_field("resourceGroupName"),
            rules: o.get_field("rules"),
        }
    }
}
