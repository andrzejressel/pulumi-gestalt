#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAttachmentArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetAttachmentFilter>,
            >,
        >,
        /// Key-value tags for the attachment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the attachment.
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAttachmentResult {
        /// ARN of the attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The state of the association (see [the underlying AWS API](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_TransitGatewayAttachmentAssociation.html) for valid values).
        pub association_state: pulumi_gestalt_rust::Output<String>,
        /// The ID of the route table for the transit gateway.
        pub association_transit_gateway_route_table_id: pulumi_gestalt_rust::Output<
            String,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetAttachmentFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the resource.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the resource.
        pub resource_owner_id: pulumi_gestalt_rust::Output<String>,
        /// Resource type.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Attachment state.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the attachment.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the transit gateway.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account that owns the transit gateway.
        pub transit_gateway_owner_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAttachmentArgs,
    ) -> GetAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getAttachment:getAttachment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAttachmentResult {
            arn: o.get_field("arn"),
            association_state: o.get_field("associationState"),
            association_transit_gateway_route_table_id: o
                .get_field("associationTransitGatewayRouteTableId"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            resource_id: o.get_field("resourceId"),
            resource_owner_id: o.get_field("resourceOwnerId"),
            resource_type: o.get_field("resourceType"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            transit_gateway_owner_id: o.get_field("transitGatewayOwnerId"),
        }
    }
}
