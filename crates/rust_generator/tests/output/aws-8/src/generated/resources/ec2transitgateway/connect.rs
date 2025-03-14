/// Manages an EC2 Transit Gateway Connect.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let attachment = connect::create(
///         "attachment",
///         ConnectArgs::builder()
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .transport_attachment_id("${example.id}")
///             .build_struct(),
///     );
///     let example = vpc_attachment::create(
///         "example",
///         VpcAttachmentArgs::builder()
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_connect` using the EC2 Transit Gateway Connect identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/connect:Connect example tgw-attach-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connect {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectArgs {
        /// The tunnel protocol. Valid values: `gre`. Default is `gre`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway Connect. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean whether the Connect should be associated with the EC2 Transit Gateway association default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_association: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean whether the Connect should propagate routes with the EC2 Transit Gateway propagation default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway.
        #[builder(into)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The underlaying VPC attachment
        #[builder(into)]
        pub transport_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectResult {
        /// The tunnel protocol. Valid values: `gre`. Default is `gre`.
        pub protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway Connect. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean whether the Connect should be associated with the EC2 Transit Gateway association default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        pub transit_gateway_default_route_table_association: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Boolean whether the Connect should propagate routes with the EC2 Transit Gateway propagation default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        pub transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The underlaying VPC attachment
        pub transport_attachment_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectArgs,
    ) -> ConnectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let protocol_binding = args.protocol.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_default_route_table_association_binding = args
            .transit_gateway_default_route_table_association
            .get_output(context);
        let transit_gateway_default_route_table_propagation_binding = args
            .transit_gateway_default_route_table_propagation
            .get_output(context);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(context);
        let transport_attachment_id_binding = args
            .transport_attachment_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/connect:Connect".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayDefaultRouteTableAssociation".into(),
                    value: &transit_gateway_default_route_table_association_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayDefaultRouteTablePropagation".into(),
                    value: &transit_gateway_default_route_table_propagation_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transportAttachmentId".into(),
                    value: &transport_attachment_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectResult {
            protocol: o.get_field("protocol"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_default_route_table_association: o
                .get_field("transitGatewayDefaultRouteTableAssociation"),
            transit_gateway_default_route_table_propagation: o
                .get_field("transitGatewayDefaultRouteTablePropagation"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            transport_attachment_id: o.get_field("transportAttachmentId"),
        }
    }
}
