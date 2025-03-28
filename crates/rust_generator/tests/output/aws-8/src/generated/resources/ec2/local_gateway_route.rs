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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_gateway_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalGatewayRouteArgs {
        /// IPv4 CIDR range used for destination matches. Routing decisions are based on the most specific match.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Local Gateway Route Table.
        #[builder(into)]
        pub local_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Local Gateway Virtual Interface Group.
        #[builder(into)]
        pub local_gateway_virtual_interface_group_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct LocalGatewayRouteResult {
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalGatewayRouteArgs,
    ) -> LocalGatewayRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_cidr_block_binding = args
            .destination_cidr_block
            .get_output(context);
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_output(context);
        let local_gateway_virtual_interface_group_id_binding = args
            .local_gateway_virtual_interface_group_id
            .get_output(context);
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
        };
        let o = context.register_resource(request);
        LocalGatewayRouteResult {
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            local_gateway_route_table_id: o.get_field("localGatewayRouteTableId"),
            local_gateway_virtual_interface_group_id: o
                .get_field("localGatewayVirtualInterfaceGroupId"),
        }
    }
}
