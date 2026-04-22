/// Manages an EC2 Transit Gateway Prefix List Reference.
///
/// ## Example Usage
///
/// ### Attachment Routing
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = prefix_list_reference::create(
///         "example",
///         PrefixListReferenceArgs::builder()
///             .prefix_list_id("${exampleAwsEc2ManagedPrefixList.id}")
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
/// ### Blackhole Routing
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = prefix_list_reference::create(
///         "example",
///         PrefixListReferenceArgs::builder()
///             .blackhole(true)
///             .prefix_list_id("${exampleAwsEc2ManagedPrefixList.id}")
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
/// Using `pulumi import`, import `aws_ec2_transit_gateway_prefix_list_reference` using the EC2 Transit Gateway Route Table identifier and EC2 Prefix List identifier, separated by an underscore (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/prefixListReference:PrefixListReference example tgw-rtb-12345678_pl-12345678
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod prefix_list_reference {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrefixListReferenceArgs {
        /// Indicates whether to drop traffic that matches the Prefix List. Defaults to `false`.
        #[builder(into, default)]
        pub blackhole: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Identifier of EC2 Prefix List.
        #[builder(into)]
        pub prefix_list_id: pulumi_gestalt_rust::Input<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct PrefixListReferenceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to drop traffic that matches the Prefix List. Defaults to `false`.
        pub blackhole: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Identifier of EC2 Prefix List.
        pub prefix_list_id: pulumi_gestalt_rust::Output<String>,
        pub prefix_list_owner_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrefixListReferenceArgs,
    ) -> PrefixListReferenceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrefixListReferenceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> PrefixListReferenceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrefixListReferenceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> PrefixListReferenceResult {
        let blackhole_binding = args.blackhole.get_output(ctx);
        let prefix_list_id_binding = args.prefix_list_id.get_output(ctx);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(ctx);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/prefixListReference:PrefixListReference"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blackhole".into(),
                    value: &blackhole_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefixListId".into(),
                    value: &prefix_list_id_binding.drop_type(),
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
        PrefixListReferenceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            blackhole: o.get_field("blackhole"),
            prefix_list_id: o.get_field("prefixListId"),
            prefix_list_owner_id: o.get_field("prefixListOwnerId"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}
