/// Requests automatic route propagation between a VPN gateway and a route table.
///
/// > **Note:** This resource should not be used with a route table that has
/// the `propagating_vgws` argument set. If that argument is set, any route
/// propagation not explicitly listed in its value will be removed.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpn_gateway_route_propagation::create(
///         "example",
///         VpnGatewayRoutePropagationArgs::builder()
///             .route_table_id("${exampleAwsRouteTable.id}")
///             .vpn_gateway_id("${exampleAwsVpnGateway.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpn_gateway_route_propagation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnGatewayRoutePropagationArgs {
        /// The id of the `aws.ec2.RouteTable` to propagate routes into.
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the `aws.ec2.VpnGateway` to propagate routes from.
        #[builder(into)]
        pub vpn_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnGatewayRoutePropagationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The id of the `aws.ec2.RouteTable` to propagate routes into.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the `aws.ec2.VpnGateway` to propagate routes from.
        pub vpn_gateway_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnGatewayRoutePropagationArgs,
    ) -> VpnGatewayRoutePropagationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnGatewayRoutePropagationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpnGatewayRoutePropagationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnGatewayRoutePropagationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpnGatewayRoutePropagationResult {
        let route_table_id_binding = args.route_table_id.get_output(ctx);
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpnGatewayRoutePropagation:VpnGatewayRoutePropagation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpnGatewayRoutePropagationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            route_table_id: o.get_field("routeTableId"),
            vpn_gateway_id: o.get_field("vpnGatewayId"),
        }
    }
}
