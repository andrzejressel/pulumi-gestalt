/// Provides a Direct Connect private virtual interface resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = private_virtual_interface::create(
///         "foo",
///         PrivateVirtualInterfaceArgs::builder()
///             .address_family("ipv4")
///             .bgp_asn(65352)
///             .connection_id("dxcon-zzzzzzzz")
///             .name("vif-foo")
///             .vlan(4094)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect private virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/privateVirtualInterface:PrivateVirtualInterface test dxvif-33cc44dd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod private_virtual_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateVirtualInterfaceArgs {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        #[builder(into)]
        pub address_family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon. Required for IPv4 BGP peers.
        #[builder(into, default)]
        pub amazon_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        #[builder(into)]
        pub bgp_asn: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The authentication key for BGP configuration.
        #[builder(into, default)]
        pub bgp_auth_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Direct Connect connection (or LAG) on which to create the virtual interface.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic. Required for IPv4 BGP peers.
        #[builder(into, default)]
        pub customer_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        #[builder(into, default)]
        pub dx_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum transmission unit (MTU) is the size, in bytes, of the largest permissible packet that can be passed over the connection.
        /// The MTU of a virtual private interface can be either `1500` or `9001` (jumbo frames). Default is `1500`.
        #[builder(into, default)]
        pub mtu: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name for the virtual interface.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to enable or disable SiteLink.
        #[builder(into, default)]
        pub sitelink_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VLAN ID.
        #[builder(into)]
        pub vlan: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the virtual private gateway to which to connect the virtual interface.
        #[builder(into, default)]
        pub vpn_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrivateVirtualInterfaceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon. Required for IPv4 BGP peers.
        pub amazon_address: pulumi_gestalt_rust::Output<String>,
        pub amazon_side_asn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the virtual interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Direct Connect endpoint on which the virtual interface terminates.
        pub aws_device: pulumi_gestalt_rust::Output<String>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        pub bgp_asn: pulumi_gestalt_rust::Output<i32>,
        /// The authentication key for BGP configuration.
        pub bgp_auth_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Direct Connect connection (or LAG) on which to create the virtual interface.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic. Required for IPv4 BGP peers.
        pub customer_address: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        pub dx_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether jumbo frames (9001 MTU) are supported.
        pub jumbo_frame_capable: pulumi_gestalt_rust::Output<bool>,
        /// The maximum transmission unit (MTU) is the size, in bytes, of the largest permissible packet that can be passed over the connection.
        /// The MTU of a virtual private interface can be either `1500` or `9001` (jumbo frames). Default is `1500`.
        pub mtu: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name for the virtual interface.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to enable or disable SiteLink.
        pub sitelink_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VLAN ID.
        pub vlan: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the virtual private gateway to which to connect the virtual interface.
        pub vpn_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateVirtualInterfaceArgs,
    ) -> PrivateVirtualInterfaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_family_binding = args.address_family.get_output(context);
        let amazon_address_binding = args.amazon_address.get_output(context);
        let bgp_asn_binding = args.bgp_asn.get_output(context);
        let bgp_auth_key_binding = args.bgp_auth_key.get_output(context);
        let connection_id_binding = args.connection_id.get_output(context);
        let customer_address_binding = args.customer_address.get_output(context);
        let dx_gateway_id_binding = args.dx_gateway_id.get_output(context);
        let mtu_binding = args.mtu.get_output(context);
        let name_binding = args.name.get_output(context);
        let sitelink_enabled_binding = args.sitelink_enabled.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vlan_binding = args.vlan.get_output(context);
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/privateVirtualInterface:PrivateVirtualInterface"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressFamily".into(),
                    value: &address_family_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amazonAddress".into(),
                    value: &amazon_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpAsn".into(),
                    value: &bgp_asn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpAuthKey".into(),
                    value: &bgp_auth_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerAddress".into(),
                    value: &customer_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dxGatewayId".into(),
                    value: &dx_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mtu".into(),
                    value: &mtu_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sitelinkEnabled".into(),
                    value: &sitelink_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vlan".into(),
                    value: &vlan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrivateVirtualInterfaceResult {
            id: o.get_field("id"),
            address_family: o.get_field("addressFamily"),
            amazon_address: o.get_field("amazonAddress"),
            amazon_side_asn: o.get_field("amazonSideAsn"),
            arn: o.get_field("arn"),
            aws_device: o.get_field("awsDevice"),
            bgp_asn: o.get_field("bgpAsn"),
            bgp_auth_key: o.get_field("bgpAuthKey"),
            connection_id: o.get_field("connectionId"),
            customer_address: o.get_field("customerAddress"),
            dx_gateway_id: o.get_field("dxGatewayId"),
            jumbo_frame_capable: o.get_field("jumboFrameCapable"),
            mtu: o.get_field("mtu"),
            name: o.get_field("name"),
            sitelink_enabled: o.get_field("sitelinkEnabled"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vlan: o.get_field("vlan"),
            vpn_gateway_id: o.get_field("vpnGatewayId"),
        }
    }
}
