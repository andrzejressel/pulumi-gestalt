/// Provides a resource, that manages Cloudflare tunnel routes for Zero
/// Trust. Tunnel routes are used to direct IP traffic through
/// Cloudflare Tunnels.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/tunnelRoute:TunnelRoute example <account_id>/<network_cidr>/<virtual_network_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod tunnel_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelRouteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the tunnel route.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the tunnel that will service the tunnel route.
        #[builder(into)]
        pub tunnel_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TunnelRouteResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the tunnel route.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the tunnel that will service the tunnel route.
        pub tunnel_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
        pub virtual_network_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelRouteArgs,
    ) -> TunnelRouteResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelRouteArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TunnelRouteResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelRouteArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TunnelRouteResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let comment_binding = args.comment.get_output(ctx);
        let network_binding = args.network.get_output(ctx);
        let tunnel_id_binding = args.tunnel_id.get_output(ctx);
        let virtual_network_id_binding = args.virtual_network_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/tunnelRoute:TunnelRoute".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tunnelId".into(),
                    value: &tunnel_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TunnelRouteResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            comment: o.get_field("comment"),
            network: o.get_field("network"),
            tunnel_id: o.get_field("tunnelId"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
