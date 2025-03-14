/// Manages a Palo Alto Next Generation Firewall Virtual Network Panorama.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource-group
///       location: westeurope
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-public-ip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       sku: Standard
///   exampleNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: example
///     properties:
///       name: example-nsg
///       location: ${test.location}
///       resourceGroupName: ${test.name}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
///   trust:
///     type: azure:network:Subnet
///     properties:
///       name: example-trust-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///       delegations:
///         - name: trusted
///           serviceDelegation:
///             name: PaloAltoNetworks.Cloudngfw/firewalls
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   trustSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: trust
///     properties:
///       subnetId: ${trust.id}
///       networkSecurityGroupId: ${exampleNetworkSecurityGroup.id}
///   untrust:
///     type: azure:network:Subnet
///     properties:
///       name: example-untrust-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       delegations:
///         - name: untrusted
///           serviceDelegation:
///             name: PaloAltoNetworks.Cloudngfw/firewalls
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   untrustSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: untrust
///     properties:
///       subnetId: ${untrust.id}
///       networkSecurityGroupId: ${exampleNetworkSecurityGroup.id}
///   exampleNextGenerationFirewallVirtualNetworkPanorama:
///     type: azure:paloalto:NextGenerationFirewallVirtualNetworkPanorama
///     name: example
///     properties:
///       name: example-ngfwvh
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       panoramaBase64Config: e2RnbmFtZTogY25nZnctYXotZXhhbXBsZSwgdHBsbmFtZTogY25nZnctZXhhbXBsZS10ZW1wbGF0ZS1zdGFjaywgZXhhbXBsZS1wYW5vcmFtYS1zZXJ2ZXI6IDE5Mi4xNjguMC4xLCB2bS1hdXRoLWtleTogMDAwMDAwMDAwMDAwMDAwLCBleHBpcnk6IDIwMjQvMDcvMzF9Cg==
///       networkProfile:
///         publicIpAddressIds:
///           - ${examplePublicIp.id}
///         vnetConfiguration:
///           virtualNetworkId: ${exampleVirtualNetwork.id}
///           trustedSubnetId: ${trust.id}
///           untrustedSubnetId: ${untrust.id}
/// ```
///
/// ## Import
///
/// Palo Alto Next Generation Firewall Virtual Network Panoramas can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/nextGenerationFirewallVirtualNetworkPanorama:NextGenerationFirewallVirtualNetworkPanorama example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/firewalls/myVNetPanoramaFW
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod next_generation_firewall_virtual_network_panorama {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualNetworkPanoramaArgs {
        /// One or more `destination_nat` blocks as defined below.
        #[builder(into, default)]
        pub destination_nats: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaDestinationNat,
                >,
            >,
        >,
        /// A `dns_settings` block as defined below.
        #[builder(into, default)]
        pub dns_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaDnsSettings,
            >,
        >,
        /// The Azure Region where the Palo Alto Next Generation Firewall Virtual Network Panorama should exist. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Palo Alto Next Generation Firewall Virtual Network Panorama. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network_profile` block as defined below.
        #[builder(into)]
        pub network_profile: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaNetworkProfile,
        >,
        /// The base64 encoded configuration registration string as defined by your Panorama Server for your Cloud Device Group.
        #[builder(into)]
        pub panorama_base64_config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Palo Alto Next Generation Firewall Virtual Network Panorama should exist. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Palo Alto Next Generation Firewall Virtual Network Panorama.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualNetworkPanoramaResult {
        /// One or more `destination_nat` blocks as defined below.
        pub destination_nats: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaDestinationNat,
                >,
            >,
        >,
        /// A `dns_settings` block as defined below.
        pub dns_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaDnsSettings,
            >,
        >,
        /// The Azure Region where the Palo Alto Next Generation Firewall Virtual Network Panorama should exist. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Palo Alto Next Generation Firewall Virtual Network Panorama. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_profile` block as defined below.
        pub network_profile: pulumi_gestalt_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaNetworkProfile,
        >,
        /// The base64 encoded configuration registration string as defined by your Panorama Server for your Cloud Device Group.
        pub panorama_base64_config: pulumi_gestalt_rust::Output<String>,
        /// A `panorama` block as defined below.
        pub panoramas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaPanorama,
            >,
        >,
        /// The name of the Resource Group where the Palo Alto Next Generation Firewall Virtual Network Panorama should exist. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Palo Alto Next Generation Firewall Virtual Network Panorama.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NextGenerationFirewallVirtualNetworkPanoramaArgs,
    ) -> NextGenerationFirewallVirtualNetworkPanoramaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_nats_binding = args.destination_nats.get_output(context);
        let dns_settings_binding = args.dns_settings.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_profile_binding = args.network_profile.get_output(context);
        let panorama_base64_config_binding = args
            .panorama_base64_config
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/nextGenerationFirewallVirtualNetworkPanorama:NextGenerationFirewallVirtualNetworkPanorama"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationNats".into(),
                    value: &destination_nats_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSettings".into(),
                    value: &dns_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "panoramaBase64Config".into(),
                    value: &panorama_base64_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NextGenerationFirewallVirtualNetworkPanoramaResult {
            destination_nats: o.get_field("destinationNats"),
            dns_settings: o.get_field("dnsSettings"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_profile: o.get_field("networkProfile"),
            panorama_base64_config: o.get_field("panoramaBase64Config"),
            panoramas: o.get_field("panoramas"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
