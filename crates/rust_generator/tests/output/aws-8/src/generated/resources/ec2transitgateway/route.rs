/// Manages an EC2 Transit Gateway Route.
///
/// ## Example Usage
///
/// ### Standard usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route::create(
///         "example",
///         RouteArgs::builder()
///             .destination_cidr_block("0.0.0.0/0")
///             .transit_gateway_attachment_id(
///                 "${exampleAwsEc2TransitGatewayVpcAttachment.id}",
///             )
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGateway.associationDefaultRouteTableId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Blackhole route
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route::create(
///         "example",
///         RouteArgs::builder()
///             .blackhole(true)
///             .destination_cidr_block("0.0.0.0/0")
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGateway.associationDefaultRouteTableId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_route` using the EC2 Transit Gateway Route Table, an underscore, and the destination. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/route:Route example tgw-rtb-12345678_0.0.0.0/0
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// Indicates whether to drop traffic that matches this route (default to `false`).
        #[builder(into, default)]
        pub blackhole: pulumi_gestalt_rust::Input<Option<bool>>,
        /// IPv4 or IPv6 RFC1924 CIDR used for destination matches. Routing decisions are based on the most specific match.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::Input<String>,
        /// Identifier of EC2 Transit Gateway Attachment (required if `blackhole` is set to false).
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Input<
            Option<String>,
        >,
        /// Identifier of EC2 Transit Gateway Route Table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to drop traffic that matches this route (default to `false`).
        pub blackhole: pulumi_gestalt_rust::Output<Option<bool>>,
        /// IPv4 or IPv6 RFC1924 CIDR used for destination matches. Routing decisions are based on the most specific match.
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment (required if `blackhole` is set to false).
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of EC2 Transit Gateway Route Table.
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteArgs,
    ) -> RouteResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RouteResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RouteResult {
        let blackhole_binding = args.blackhole.get_output(ctx);
        let destination_cidr_block_binding = args.destination_cidr_block.get_output(ctx);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(ctx);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blackhole".into(),
                    value: &blackhole_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: &transit_gateway_route_table_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RouteResult {
            id: o.get_id(),
            urn: o.get_urn(),
            blackhole: o.get_field("blackhole"),
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}
