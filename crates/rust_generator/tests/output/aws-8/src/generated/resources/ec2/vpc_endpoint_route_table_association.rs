/// Manages a VPC Endpoint Route Table Association
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_endpoint_route_table_association::create(
///         "example",
///         VpcEndpointRouteTableAssociationArgs::builder()
///             .route_table_id("${exampleAwsRouteTable.id}")
///             .vpc_endpoint_id("${exampleAwsVpcEndpoint.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Route Table Associations using `vpc_endpoint_id` together with `route_table_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointRouteTableAssociation:VpcEndpointRouteTableAssociation example vpce-aaaaaaaa/rtb-bbbbbbbb
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_endpoint_route_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointRouteTableAssociationArgs {
        /// Identifier of the EC2 Route Table to be associated with the VPC Endpoint.
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::Input<String>,
        /// Identifier of the VPC Endpoint with which the EC2 Route Table will be associated.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointRouteTableAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the EC2 Route Table to be associated with the VPC Endpoint.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the VPC Endpoint with which the EC2 Route Table will be associated.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointRouteTableAssociationArgs,
    ) -> VpcEndpointRouteTableAssociationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointRouteTableAssociationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcEndpointRouteTableAssociationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointRouteTableAssociationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcEndpointRouteTableAssociationResult {
        let route_table_id_binding = args.route_table_id.get_output(ctx);
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointRouteTableAssociation:VpcEndpointRouteTableAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcEndpointRouteTableAssociationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            route_table_id: o.get_field("routeTableId"),
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
        }
    }
}
