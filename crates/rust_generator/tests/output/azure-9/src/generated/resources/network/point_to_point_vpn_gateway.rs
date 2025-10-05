/// Manages a Point-to-Site VPN Gateway.
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
///     let examplePointToPointVpnGateway = point_to_point_vpn_gateway::create(
///         "examplePointToPointVpnGateway",
///         PointToPointVpnGatewayArgs::builder()
///             .connection_configurations(
///                 vec![
///                     PointToPointVpnGatewayConnectionConfiguration::builder()
///                     .name("example-gateway-config")
///                     .vpnClientAddressPool(PointToPointVpnGatewayConnectionConfigurationVpnClientAddressPool::builder()
///                     .addressPrefixes(vec!["10.0.2.0/24",]).build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-vpn-gateway")
///             .resource_group_name("${example.name}")
///             .scale_unit(1)
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .vpn_server_configuration_id("${exampleVpnServerConfiguration.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.0.0/23")
///             .location("${example.location}")
///             .name("example-virtualhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-virtualwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVpnServerConfiguration = vpn_server_configuration::create(
///         "exampleVpnServerConfiguration",
///         VpnServerConfigurationArgs::builder()
///             .client_root_certificates(
///                 vec![
///                     VpnServerConfigurationClientRootCertificate::builder()
///                     .name("DigiCert-Federated-ID-Root-CA")
///                     .publicCertData("MIIDuzCCAqOgAwIBAgIQCHTZWCM+IlfFIRXIvyKSrjANBgkqhkiG9w0BAQsFADBn\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSYwJAYDVQQDEx1EaWdpQ2VydCBGZWRlcmF0ZWQgSUQg\nUm9vdCBDQTAeFw0xMzAxMTUxMjAwMDBaFw0zMzAxMTUxMjAwMDBaMGcxCzAJBgNV\nBAYTAlVTMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdp\nY2VydC5jb20xJjAkBgNVBAMTHURpZ2lDZXJ0IEZlZGVyYXRlZCBJRCBSb290IENB\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvAEB4pcCqnNNOWE6Ur5j\nQPUH+1y1F9KdHTRSza6k5iDlXq1kGS1qAkuKtw9JsiNRrjltmFnzMZRBbX8Tlfl8\nzAhBmb6dDduDGED01kBsTkgywYPxXVTKec0WxYEEF0oMn4wSYNl0lt2eJAKHXjNf\nGTwiibdP8CUR2ghSM2sUTI8Nt1Omfc4SMHhGhYD64uJMbX98THQ/4LMGuYegou+d\nGTiahfHtjn7AboSEknwAMJHCh5RlYZZ6B1O4QbKJ+34Q0eKgnI3X6Vc9u0zf6DH8\nDk+4zQDYRRTqTnVO3VT8jzqDlCRuNtq6YvryOWN74/dq8LQhUnXHvFyrsdMaE1X2\nDwIDAQABo2MwYTAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV\nHQ4EFgQUGRdkFnbGt1EWjKwbUne+5OaZvRYwHwYDVR0jBBgwFoAUGRdkFnbGt1EW\njKwbUne+5OaZvRYwDQYJKoZIhvcNAQELBQADggEBAHcqsHkrjpESqfuVTRiptJfP\n9JbdtWqRTmOf6uJi2c8YVqI6XlKXsD8C1dUUaaHKLUJzvKiazibVuBwMIT84AyqR\nQELn3e0BtgEymEygMU569b01ZPxoFSnNXc7qDZBDef8WfqAV/sxkTi8L9BkmFYfL\nuGLOhRJOFprPdoDIUBB+tmCl3oDcBy3vnUeOEioz8zAkprcb3GHwHAK+vHmmfgcn\nWsfMLH4JCLa/tRYL+Rw/N3ybCkDp00s0WUZ+AoDywSl0Q/ZEnNY0MsFiw6LyIdbq\nM/s/1JRtO3bDSzD9TazRVzn2oBqzSa8VgIo5C1nOnoAKJTlsClJKvIhnRlaLQqk=\n")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-config")
///             .resource_group_name("${example.name}")
///             .vpn_authentication_types(vec!["Certificate",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Point-to-Site VPN Gateway's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/pointToPointVpnGateway:PointToPointVpnGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/p2sVpnGateways/gateway1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod point_to_point_vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PointToPointVpnGatewayArgs {
        /// A `connection_configuration` block as defined below.
        #[builder(into)]
        pub connection_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::network::PointToPointVpnGatewayConnectionConfiguration,
            >,
        >,
        /// A list of IP Addresses of DNS Servers for the Point-to-Site VPN Gateway.
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the Routing Preference for the Public IP Interface of the VPN Gateway enabled? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub routing_preference_internet_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The [Scale Unit](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-faq#what-is-a-virtual-wan-gateway-scale-unit) for this Point-to-Site VPN Gateway.
        #[builder(into)]
        pub scale_unit: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A mapping of tags to assign to the Point-to-Site VPN Gateway.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub where this Point-to-Site VPN Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPN Server Configuration which this Point-to-Site VPN Gateway should use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vpn_server_configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PointToPointVpnGatewayResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `connection_configuration` block as defined below.
        pub connection_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::network::PointToPointVpnGatewayConnectionConfiguration,
            >,
        >,
        /// A list of IP Addresses of DNS Servers for the Point-to-Site VPN Gateway.
        pub dns_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is the Routing Preference for the Public IP Interface of the VPN Gateway enabled? Defaults to `false`. Changing this forces a new resource to be created.
        pub routing_preference_internet_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The [Scale Unit](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-faq#what-is-a-virtual-wan-gateway-scale-unit) for this Point-to-Site VPN Gateway.
        pub scale_unit: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags to assign to the Point-to-Site VPN Gateway.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub where this Point-to-Site VPN Gateway should exist. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPN Server Configuration which this Point-to-Site VPN Gateway should use. Changing this forces a new resource to be created.
        pub vpn_server_configuration_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PointToPointVpnGatewayArgs,
    ) -> PointToPointVpnGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_configurations_binding = args
            .connection_configurations
            .get_output(context);
        let dns_servers_binding = args.dns_servers.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let routing_preference_internet_enabled_binding = args
            .routing_preference_internet_enabled
            .get_output(context);
        let scale_unit_binding = args.scale_unit.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context);
        let vpn_server_configuration_id_binding = args
            .vpn_server_configuration_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/pointToPointVpnGateway:PointToPointVpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionConfigurations".into(),
                    value: &connection_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding.drop_type(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingPreferenceInternetEnabled".into(),
                    value: &routing_preference_internet_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scaleUnit".into(),
                    value: &scale_unit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnServerConfigurationId".into(),
                    value: &vpn_server_configuration_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PointToPointVpnGatewayResult {
            id: o.get_field("id"),
            connection_configurations: o.get_field("connectionConfigurations"),
            dns_servers: o.get_field("dnsServers"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            routing_preference_internet_enabled: o
                .get_field("routingPreferenceInternetEnabled"),
            scale_unit: o.get_field("scaleUnit"),
            tags: o.get_field("tags"),
            virtual_hub_id: o.get_field("virtualHubId"),
            vpn_server_configuration_id: o.get_field("vpnServerConfigurationId"),
        }
    }
}
