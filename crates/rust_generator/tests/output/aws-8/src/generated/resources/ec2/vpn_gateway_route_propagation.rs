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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        /// The id of the `aws.ec2.RouteTable` to propagate routes into.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the `aws.ec2.VpnGateway` to propagate routes from.
        pub vpn_gateway_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnGatewayRoutePropagationArgs,
    ) -> VpnGatewayRoutePropagationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let route_table_id_binding = args.route_table_id.get_output(context);
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(context);
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
        };
        let o = context.register_resource(request);
        VpnGatewayRoutePropagationResult {
            route_table_id: o.get_field("routeTableId"),
            vpn_gateway_id: o.get_field("vpnGatewayId"),
        }
    }
}
