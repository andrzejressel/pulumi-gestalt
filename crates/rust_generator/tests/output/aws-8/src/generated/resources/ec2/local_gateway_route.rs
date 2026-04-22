/// Manages an EC2 Local Gateway Route. More information can be found in the [Outposts User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#routing).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = local_gateway_route::create(
///         "example",
///         LocalGatewayRouteArgs::builder()
///             .destination_cidr_block("172.16.0.0/16")
///             .local_gateway_route_table_id("${exampleAwsEc2LocalGatewayRouteTable.id}")
///             .local_gateway_virtual_interface_group_id(
///                 "${exampleAwsEc2LocalGatewayVirtualInterfaceGroup.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_local_gateway_route` using the EC2 Local Gateway Route Table identifier and destination CIDR block separated by underscores (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/localGatewayRoute:LocalGatewayRoute example lgw-rtb-12345678_172.16.0.0/16
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod local_gateway_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalGatewayRouteArgs {
        /// IPv4 CIDR range used for destination matches. Routing decisions are based on the most specific match.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::Input<String>,
        /// Identifier of EC2 Local Gateway Route Table.
        #[builder(into)]
        pub local_gateway_route_table_id: pulumi_gestalt_rust::Input<String>,
        /// Identifier of EC2 Local Gateway Virtual Interface Group.
        #[builder(into)]
        pub local_gateway_virtual_interface_group_id: pulumi_gestalt_rust::Input<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct LocalGatewayRouteResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// IPv4 CIDR range used for destination matches. Routing decisions are based on the most specific match.
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Route Table.
        pub local_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Virtual Interface Group.
        pub local_gateway_virtual_interface_group_id: pulumi_gestalt_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalGatewayRouteArgs,
    ) -> LocalGatewayRouteResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalGatewayRouteArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LocalGatewayRouteResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalGatewayRouteArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LocalGatewayRouteResult {
        let destination_cidr_block_binding = args.destination_cidr_block.get_output(ctx);
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_output(ctx);
        let local_gateway_virtual_interface_group_id_binding = args
            .local_gateway_virtual_interface_group_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/localGatewayRoute:LocalGatewayRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localGatewayRouteTableId".into(),
                    value: &local_gateway_route_table_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localGatewayVirtualInterfaceGroupId".into(),
                    value: &local_gateway_virtual_interface_group_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LocalGatewayRouteResult {
            id: o.get_id(),
            urn: o.get_urn(),
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            local_gateway_route_table_id: o.get_field("localGatewayRouteTableId"),
            local_gateway_virtual_interface_group_id: o
                .get_field("localGatewayVirtualInterfaceGroupId"),
        }
    }
}
