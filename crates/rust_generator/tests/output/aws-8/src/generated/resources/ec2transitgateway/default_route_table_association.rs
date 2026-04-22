/// Resource for managing an AWS EC2 (Elastic Compute Cloud) Transit Gateway Default Route Table Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = default_route_table_association::create(
///         "example",
///         DefaultRouteTableAssociationArgs::builder()
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGatewayRouteTable.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod default_route_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultRouteTableAssociationArgs {
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<
                super::super::types::ec2transitgateway::DefaultRouteTableAssociationTimeouts,
            >,
        >,
        /// ID of the Transit Gateway to change the default association route table on.
        #[builder(into)]
        pub transit_gateway_id: pulumi_gestalt_rust::Input<String>,
        /// ID of the Transit Gateway Route Table to be made the default association route table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultRouteTableAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub original_default_route_table_id: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ec2transitgateway::DefaultRouteTableAssociationTimeouts,
            >,
        >,
        /// ID of the Transit Gateway to change the default association route table on.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the Transit Gateway Route Table to be made the default association route table.
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultRouteTableAssociationArgs,
    ) -> DefaultRouteTableAssociationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultRouteTableAssociationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DefaultRouteTableAssociationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultRouteTableAssociationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DefaultRouteTableAssociationResult {
        let timeouts_binding = args.timeouts.get_output(ctx);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(ctx);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/defaultRouteTableAssociation:DefaultRouteTableAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: &transit_gateway_route_table_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DefaultRouteTableAssociationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            original_default_route_table_id: o.get_field("originalDefaultRouteTableId"),
            timeouts: o.get_field("timeouts"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}
