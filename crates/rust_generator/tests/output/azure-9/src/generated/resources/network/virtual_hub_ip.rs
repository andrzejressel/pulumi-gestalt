/// Manages a Virtual Hub IP. This resource is also known as a Route Server.
///
/// > **NOTE** Virtual Hub IP only supports Standard Virtual Hub without Virtual Wan.
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
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("example-pip")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.5.1.0/24",])
///             .name("RouteServerSubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleVirtualHubIp = virtual_hub_ip::create(
///         "exampleVirtualHubIp",
///         VirtualHubIpArgs::builder()
///             .name("example-vhubipconfig")
///             .private_ip_address("10.5.1.18")
///             .private_ip_allocation_method("Static")
///             .public_ip_address_id("${examplePublicIp.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.5.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub IPs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHubIp:VirtualHubIp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/virtualHub1/ipConfigurations/ipConfig1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_hub_ip {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubIpArgs {
        /// The name which should be used for this Virtual Hub IP. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The private IP address of the IP configuration.
        #[builder(into, default)]
        pub private_ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The private IP address allocation method. Possible values are `Static` and `Dynamic` is allowed. Defaults to `Dynamic`.
        #[builder(into, default)]
        pub private_ip_allocation_method: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Public IP Address. This option is required since September 1st 2021. Changing this forces a new resource to be created.
        #[builder(into)]
        pub public_ip_address_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet that the IP will reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Virtual Hub within which this IP configuration should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubIpResult {
        /// The name which should be used for this Virtual Hub IP. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The private IP address of the IP configuration.
        pub private_ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The private IP address allocation method. Possible values are `Static` and `Dynamic` is allowed. Defaults to `Dynamic`.
        pub private_ip_allocation_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Public IP Address. This option is required since September 1st 2021. Changing this forces a new resource to be created.
        pub public_ip_address_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet that the IP will reside. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Hub within which this IP configuration should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualHubIpArgs,
    ) -> VirtualHubIpResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let private_ip_address_binding = args.private_ip_address.get_output(context);
        let private_ip_allocation_method_binding = args
            .private_ip_allocation_method
            .get_output(context);
        let public_ip_address_id_binding = args.public_ip_address_id.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualHubIp:VirtualHubIp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpAddress".into(),
                    value: &private_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpAllocationMethod".into(),
                    value: &private_ip_allocation_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpAddressId".into(),
                    value: &public_ip_address_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualHubIpResult {
            name: o.get_field("name"),
            private_ip_address: o.get_field("privateIpAddress"),
            private_ip_allocation_method: o.get_field("privateIpAllocationMethod"),
            public_ip_address_id: o.get_field("publicIpAddressId"),
            subnet_id: o.get_field("subnetId"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
