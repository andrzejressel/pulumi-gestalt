/// Manages a Virtual Network Gateway to establish secure, cross-premises connectivity.
///
/// > **Note:** Please be aware that provisioning a Virtual Network Gateway takes a long time (between 30 minutes and 1 hour)
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
///         ResourceGroupArgs::builder().location("West Europe").name("test").build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Dynamic")
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("GatewaySubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetworkGateway = virtual_network_gateway::create(
///         "exampleVirtualNetworkGateway",
///         VirtualNetworkGatewayArgs::builder()
///             .active_active(false)
///             .enable_bgp(false)
///             .ip_configurations(
///                 vec![
///                     VirtualNetworkGatewayIpConfiguration::builder()
///                     .name("vnetGatewayConfig").privateIpAddressAllocation("Dynamic")
///                     .publicIpAddressId("${examplePublicIp.id}")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .sku("Basic")
///             .type_("Vpn")
///             .vpn_client_configuration(
///                 VirtualNetworkGatewayVpnClientConfiguration::builder()
///                     .addressSpaces(vec!["10.2.0.0/24",])
///                     .revokedCertificates(
///                         vec![
///                             VirtualNetworkGatewayVpnClientConfigurationRevokedCertificate::builder()
///                             .name("Verizon-Global-Root-CA")
///                             .thumbprint("912198EEF23DCAC40939312FEE97DD560BAE49B1")
///                             .build_struct(),
///                         ],
///                     )
///                     .rootCertificates(
///                         vec![
///                             VirtualNetworkGatewayVpnClientConfigurationRootCertificate::builder()
///                             .name("DigiCert-Federated-ID-Root-CA")
///                             .publicCertData("MIIDuzCCAqOgAwIBAgIQCHTZWCM+IlfFIRXIvyKSrjANBgkqhkiG9w0BAQsFADBn\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSYwJAYDVQQDEx1EaWdpQ2VydCBGZWRlcmF0ZWQgSUQg\nUm9vdCBDQTAeFw0xMzAxMTUxMjAwMDBaFw0zMzAxMTUxMjAwMDBaMGcxCzAJBgNV\nBAYTAlVTMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdp\nY2VydC5jb20xJjAkBgNVBAMTHURpZ2lDZXJ0IEZlZGVyYXRlZCBJRCBSb290IENB\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvAEB4pcCqnNNOWE6Ur5j\nQPUH+1y1F9KdHTRSza6k5iDlXq1kGS1qAkuKtw9JsiNRrjltmFnzMZRBbX8Tlfl8\nzAhBmb6dDduDGED01kBsTkgywYPxXVTKec0WxYEEF0oMn4wSYNl0lt2eJAKHXjNf\nGTwiibdP8CUR2ghSM2sUTI8Nt1Omfc4SMHhGhYD64uJMbX98THQ/4LMGuYegou+d\nGTiahfHtjn7AboSEknwAMJHCh5RlYZZ6B1O4QbKJ+34Q0eKgnI3X6Vc9u0zf6DH8\nDk+4zQDYRRTqTnVO3VT8jzqDlCRuNtq6YvryOWN74/dq8LQhUnXHvFyrsdMaE1X2\nDwIDAQABo2MwYTAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV\nHQ4EFgQUGRdkFnbGt1EWjKwbUne+5OaZvRYwHwYDVR0jBBgwFoAUGRdkFnbGt1EW\njKwbUne+5OaZvRYwDQYJKoZIhvcNAQELBQADggEBAHcqsHkrjpESqfuVTRiptJfP\n9JbdtWqRTmOf6uJi2c8YVqI6XlKXsD8C1dUUaaHKLUJzvKiazibVuBwMIT84AyqR\nQELn3e0BtgEymEygMU569b01ZPxoFSnNXc7qDZBDef8WfqAV/sxkTi8L9BkmFYfL\nuGLOhRJOFprPdoDIUBB+tmCl3oDcBy3vnUeOEioz8zAkprcb3GHwHAK+vHmmfgcn\nWsfMLH4JCLa/tRYL+Rw/N3ybCkDp00s0WUZ+AoDywSl0Q/ZEnNY0MsFiw6LyIdbq\nM/s/1JRtO3bDSzD9TazRVzn2oBqzSa8VgIo5C1nOnoAKJTlsClJKvIhnRlaLQqk=\n")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .vpn_type("RouteBased")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Network Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetworkGateway:VirtualNetworkGateway exampleGateway /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.Network/virtualNetworkGateways/myGateway1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayArgs {
        /// If `true`, an active-active Virtual Network Gateway will be created. An active-active gateway requires a `HighPerformance` or an `UltraPerformance` SKU. If `false`, an active-standby gateway will be created. Defaults to `false`.
        #[builder(into, default)]
        pub active_active: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is BGP Route Translation for NAT enabled? Defaults to `false`.
        #[builder(into, default)]
        pub bgp_route_translation_for_nat_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `bgp_settings` block which is documented below. In this block the BGP specific settings can be defined.
        #[builder(into, default)]
        pub bgp_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VirtualNetworkGatewayBgpSettings>,
        >,
        /// A `custom_route` block as defined below. Specifies a custom routes address space for a virtual network gateway and a VpnClient.
        #[builder(into, default)]
        pub custom_route: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VirtualNetworkGatewayCustomRoute>,
        >,
        /// The ID of the local network gateway through which outbound Internet traffic from the virtual network in which the gateway is created will be routed (*forced tunnelling*). Refer to the [Azure documentation on forced tunnelling](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-forced-tunneling-rm). If not specified, forced tunnelling is disabled.
        #[builder(into, default)]
        pub default_local_network_gateway_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Is DNS forwarding enabled?
        #[builder(into, default)]
        pub dns_forwarding_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network Gateway should exist. Changing this forces a new Virtual Network Gateway to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If `true`, BGP (Border Gateway Protocol) will be enabled for this Virtual Network Gateway. Defaults to `false`.
        #[builder(into, default)]
        pub enable_bgp: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Generation of the Virtual Network gateway. Possible values include `Generation1`, `Generation2` or `None`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The available values depend on the `type` and `sku` arguments - where `Generation2` is only value for a `sku` larger than `VpnGw2` or `VpnGw2AZ`.
        #[builder(into, default)]
        pub generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more (up to 3) `ip_configuration` blocks documented below. Changing this forces a new resource to be created.
        /// An active-standby gateway requires exactly one `ip_configuration` block,
        /// an active-active gateway requires exactly two `ip_configuration` blocks whereas
        /// an active-active zone redundant gateway with P2S configuration requires exactly three `ip_configuration` blocks.
        #[builder(into)]
        pub ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::VirtualNetworkGatewayIpConfiguration>,
        >,
        /// Is IP Sec Replay Protection enabled? Defaults to `true`.
        #[builder(into, default)]
        pub ip_sec_replay_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The location/region where the Virtual Network Gateway is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Virtual Network Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `policy_group` blocks as defined below.
        #[builder(into, default)]
        pub policy_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroup>>,
        >,
        /// Should private IP be enabled on this gateway for connections? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_ip_address_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from other Azure Virtual Networks enabled? Defaults to `false`.
        #[builder(into, default)]
        pub remote_vnet_traffic_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the Virtual Network Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration of the size and capacity of the virtual network gateway. Valid options are `Basic`, `Standard`, `HighPerformance`, `UltraPerformance`, `ErGw1AZ`, `ErGw2AZ`, `ErGw3AZ`, `VpnGw1`, `VpnGw2`, `VpnGw3`, `VpnGw4`,`VpnGw5`, `VpnGw1AZ`, `VpnGw2AZ`, `VpnGw3AZ`,`VpnGw4AZ` and `VpnGw5AZ` and depend on the `type`, `vpn_type` and `generation` arguments. A `PolicyBased` gateway only supports the `Basic` SKU. Further, the `UltraPerformance` SKU is only supported by an `ExpressRoute` gateway.
        ///
        /// > **NOTE:** To build a UltraPerformance ExpressRoute Virtual Network gateway, the associated Public IP needs to be SKU "Basic" not "Standard"
        ///
        /// > **NOTE:** Not all SKUs (e.g. `ErGw1AZ`) are available in all regions. If you see `StatusCode=400 -- Original Error: Code="InvalidGatewaySkuSpecifiedForGatewayDeploymentType"` please try another region.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Network Gateway. Valid options are `Vpn` or `ExpressRoute`. Changing the type forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from remote Virtual WAN networks enabled? Defaults to `false`.
        #[builder(into, default)]
        pub virtual_wan_traffic_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `vpn_client_configuration` block which is documented below. In this block the Virtual Network Gateway can be configured to accept IPSec point-to-site connections.
        #[builder(into, default)]
        pub vpn_client_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::network::VirtualNetworkGatewayVpnClientConfiguration,
            >,
        >,
        /// The routing type of the Virtual Network Gateway. Valid options are `RouteBased` or `PolicyBased`. Defaults to `RouteBased`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub vpn_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayResult {
        /// If `true`, an active-active Virtual Network Gateway will be created. An active-active gateway requires a `HighPerformance` or an `UltraPerformance` SKU. If `false`, an active-standby gateway will be created. Defaults to `false`.
        pub active_active: pulumi_gestalt_rust::Output<bool>,
        /// Is BGP Route Translation for NAT enabled? Defaults to `false`.
        pub bgp_route_translation_for_nat_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A `bgp_settings` block which is documented below. In this block the BGP specific settings can be defined.
        pub bgp_settings: pulumi_gestalt_rust::Output<
            super::super::types::network::VirtualNetworkGatewayBgpSettings,
        >,
        /// A `custom_route` block as defined below. Specifies a custom routes address space for a virtual network gateway and a VpnClient.
        pub custom_route: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::VirtualNetworkGatewayCustomRoute>,
        >,
        /// The ID of the local network gateway through which outbound Internet traffic from the virtual network in which the gateway is created will be routed (*forced tunnelling*). Refer to the [Azure documentation on forced tunnelling](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-forced-tunneling-rm). If not specified, forced tunnelling is disabled.
        pub default_local_network_gateway_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Is DNS forwarding enabled?
        pub dns_forwarding_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network Gateway should exist. Changing this forces a new Virtual Network Gateway to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// If `true`, BGP (Border Gateway Protocol) will be enabled for this Virtual Network Gateway. Defaults to `false`.
        pub enable_bgp: pulumi_gestalt_rust::Output<bool>,
        /// The Generation of the Virtual Network gateway. Possible values include `Generation1`, `Generation2` or `None`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The available values depend on the `type` and `sku` arguments - where `Generation2` is only value for a `sku` larger than `VpnGw2` or `VpnGw2AZ`.
        pub generation: pulumi_gestalt_rust::Output<String>,
        /// One or more (up to 3) `ip_configuration` blocks documented below. Changing this forces a new resource to be created.
        /// An active-standby gateway requires exactly one `ip_configuration` block,
        /// an active-active gateway requires exactly two `ip_configuration` blocks whereas
        /// an active-active zone redundant gateway with P2S configuration requires exactly three `ip_configuration` blocks.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::VirtualNetworkGatewayIpConfiguration>,
        >,
        /// Is IP Sec Replay Protection enabled? Defaults to `true`.
        pub ip_sec_replay_protection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location/region where the Virtual Network Gateway is located. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Virtual Network Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `policy_group` blocks as defined below.
        pub policy_groups: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroup>>,
        >,
        /// Should private IP be enabled on this gateway for connections? Changing this forces a new resource to be created.
        pub private_ip_address_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from other Azure Virtual Networks enabled? Defaults to `false`.
        pub remote_vnet_traffic_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Virtual Network Gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration of the size and capacity of the virtual network gateway. Valid options are `Basic`, `Standard`, `HighPerformance`, `UltraPerformance`, `ErGw1AZ`, `ErGw2AZ`, `ErGw3AZ`, `VpnGw1`, `VpnGw2`, `VpnGw3`, `VpnGw4`,`VpnGw5`, `VpnGw1AZ`, `VpnGw2AZ`, `VpnGw3AZ`,`VpnGw4AZ` and `VpnGw5AZ` and depend on the `type`, `vpn_type` and `generation` arguments. A `PolicyBased` gateway only supports the `Basic` SKU. Further, the `UltraPerformance` SKU is only supported by an `ExpressRoute` gateway.
        ///
        /// > **NOTE:** To build a UltraPerformance ExpressRoute Virtual Network gateway, the associated Public IP needs to be SKU "Basic" not "Standard"
        ///
        /// > **NOTE:** Not all SKUs (e.g. `ErGw1AZ`) are available in all regions. If you see `StatusCode=400 -- Original Error: Code="InvalidGatewaySkuSpecifiedForGatewayDeploymentType"` please try another region.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Network Gateway. Valid options are `Vpn` or `ExpressRoute`. Changing the type forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from remote Virtual WAN networks enabled? Defaults to `false`.
        pub virtual_wan_traffic_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `vpn_client_configuration` block which is documented below. In this block the Virtual Network Gateway can be configured to accept IPSec point-to-site connections.
        pub vpn_client_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayVpnClientConfiguration,
            >,
        >,
        /// The routing type of the Virtual Network Gateway. Valid options are `RouteBased` or `PolicyBased`. Defaults to `RouteBased`. Changing this forces a new resource to be created.
        pub vpn_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkGatewayArgs,
    ) -> VirtualNetworkGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let active_active_binding = args.active_active.get_output(context);
        let bgp_route_translation_for_nat_enabled_binding = args
            .bgp_route_translation_for_nat_enabled
            .get_output(context);
        let bgp_settings_binding = args.bgp_settings.get_output(context);
        let custom_route_binding = args.custom_route.get_output(context);
        let default_local_network_gateway_id_binding = args
            .default_local_network_gateway_id
            .get_output(context);
        let dns_forwarding_enabled_binding = args
            .dns_forwarding_enabled
            .get_output(context);
        let edge_zone_binding = args.edge_zone.get_output(context);
        let enable_bgp_binding = args.enable_bgp.get_output(context);
        let generation_binding = args.generation.get_output(context);
        let ip_configurations_binding = args.ip_configurations.get_output(context);
        let ip_sec_replay_protection_enabled_binding = args
            .ip_sec_replay_protection_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_groups_binding = args.policy_groups.get_output(context);
        let private_ip_address_enabled_binding = args
            .private_ip_address_enabled
            .get_output(context);
        let remote_vnet_traffic_enabled_binding = args
            .remote_vnet_traffic_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let virtual_wan_traffic_enabled_binding = args
            .virtual_wan_traffic_enabled
            .get_output(context);
        let vpn_client_configuration_binding = args
            .vpn_client_configuration
            .get_output(context);
        let vpn_type_binding = args.vpn_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkGateway:VirtualNetworkGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activeActive".into(),
                    value: &active_active_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpRouteTranslationForNatEnabled".into(),
                    value: &bgp_route_translation_for_nat_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpSettings".into(),
                    value: &bgp_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customRoute".into(),
                    value: &custom_route_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultLocalNetworkGatewayId".into(),
                    value: &default_local_network_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsForwardingEnabled".into(),
                    value: &dns_forwarding_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableBgp".into(),
                    value: &enable_bgp_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "generation".into(),
                    value: &generation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipConfigurations".into(),
                    value: &ip_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipSecReplayProtectionEnabled".into(),
                    value: &ip_sec_replay_protection_enabled_binding.drop_type(),
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
                    name: "policyGroups".into(),
                    value: &policy_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpAddressEnabled".into(),
                    value: &private_ip_address_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteVnetTrafficEnabled".into(),
                    value: &remote_vnet_traffic_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualWanTrafficEnabled".into(),
                    value: &virtual_wan_traffic_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnClientConfiguration".into(),
                    value: &vpn_client_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnType".into(),
                    value: &vpn_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkGatewayResult {
            active_active: o.get_field("activeActive"),
            bgp_route_translation_for_nat_enabled: o
                .get_field("bgpRouteTranslationForNatEnabled"),
            bgp_settings: o.get_field("bgpSettings"),
            custom_route: o.get_field("customRoute"),
            default_local_network_gateway_id: o
                .get_field("defaultLocalNetworkGatewayId"),
            dns_forwarding_enabled: o.get_field("dnsForwardingEnabled"),
            edge_zone: o.get_field("edgeZone"),
            enable_bgp: o.get_field("enableBgp"),
            generation: o.get_field("generation"),
            ip_configurations: o.get_field("ipConfigurations"),
            ip_sec_replay_protection_enabled: o
                .get_field("ipSecReplayProtectionEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            policy_groups: o.get_field("policyGroups"),
            private_ip_address_enabled: o.get_field("privateIpAddressEnabled"),
            remote_vnet_traffic_enabled: o.get_field("remoteVnetTrafficEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            virtual_wan_traffic_enabled: o.get_field("virtualWanTrafficEnabled"),
            vpn_client_configuration: o.get_field("vpnClientConfiguration"),
            vpn_type: o.get_field("vpnType"),
        }
    }
}
