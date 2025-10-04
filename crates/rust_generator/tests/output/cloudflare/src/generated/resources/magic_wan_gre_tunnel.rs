/// Provides a resource, that manages GRE tunnels for Magic Transit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = magic_wan_gre_tunnel::create(
///         "example",
///         MagicWanGreTunnelArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .cloudflare_gre_endpoint("203.0.113.2")
///             .customer_gre_endpoint("203.0.113.1")
///             .description("Tunnel for ISP X")
///             .health_check_enabled(true)
///             .health_check_target("203.0.113.1")
///             .health_check_type("reply")
///             .interface_address("192.0.2.0/31")
///             .mtu(1476)
///             .name("GRE_1")
///             .ttl(64)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/magicWanGreTunnel:MagicWanGreTunnel example <account_id>/<tunnel_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod magic_wan_gre_tunnel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MagicWanGreTunnelArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP address assigned to the Cloudflare side of the GRE tunnel.
        #[builder(into)]
        pub cloudflare_gre_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IP address assigned to the customer side of the GRE tunnel.
        #[builder(into)]
        pub customer_gre_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the GRE tunnel intent.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if ICMP tunnel health checks are enabled.
        #[builder(into, default)]
        pub health_check_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The IP address of the customer endpoint that will receive tunnel health checks.
        #[builder(into, default)]
        pub health_check_target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
        #[builder(into, default)]
        pub health_check_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
        #[builder(into)]
        pub interface_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
        #[builder(into, default)]
        pub mtu: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name of the GRE tunnel.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Time To Live (TTL) in number of hops of the GRE tunnel.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct MagicWanGreTunnelResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IP address assigned to the Cloudflare side of the GRE tunnel.
        pub cloudflare_gre_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The IP address assigned to the customer side of the GRE tunnel.
        pub customer_gre_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Description of the GRE tunnel intent.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if ICMP tunnel health checks are enabled.
        pub health_check_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The IP address of the customer endpoint that will receive tunnel health checks.
        pub health_check_target: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
        pub health_check_type: pulumi_gestalt_rust::Output<String>,
        /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
        pub interface_address: pulumi_gestalt_rust::Output<String>,
        /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
        pub mtu: pulumi_gestalt_rust::Output<i32>,
        /// Name of the GRE tunnel.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Time To Live (TTL) in number of hops of the GRE tunnel.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MagicWanGreTunnelArgs,
    ) -> MagicWanGreTunnelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let cloudflare_gre_endpoint_binding = args
            .cloudflare_gre_endpoint
            .get_output(context);
        let customer_gre_endpoint_binding = args
            .customer_gre_endpoint
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let health_check_enabled_binding = args.health_check_enabled.get_output(context);
        let health_check_target_binding = args.health_check_target.get_output(context);
        let health_check_type_binding = args.health_check_type.get_output(context);
        let interface_address_binding = args.interface_address.get_output(context);
        let mtu_binding = args.mtu.get_output(context);
        let name_binding = args.name.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/magicWanGreTunnel:MagicWanGreTunnel".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudflareGreEndpoint".into(),
                    value: &cloudflare_gre_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerGreEndpoint".into(),
                    value: &customer_gre_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckEnabled".into(),
                    value: &health_check_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckTarget".into(),
                    value: &health_check_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckType".into(),
                    value: &health_check_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interfaceAddress".into(),
                    value: &interface_address_binding.drop_type(),
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
                    name: "ttl".into(),
                    value: &ttl_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MagicWanGreTunnelResult {
            id: o.get_field("id"),
            account_id: o.get_field("accountId"),
            cloudflare_gre_endpoint: o.get_field("cloudflareGreEndpoint"),
            customer_gre_endpoint: o.get_field("customerGreEndpoint"),
            description: o.get_field("description"),
            health_check_enabled: o.get_field("healthCheckEnabled"),
            health_check_target: o.get_field("healthCheckTarget"),
            health_check_type: o.get_field("healthCheckType"),
            interface_address: o.get_field("interfaceAddress"),
            mtu: o.get_field("mtu"),
            name: o.get_field("name"),
            ttl: o.get_field("ttl"),
        }
    }
}
