/// Resource for managing an AWS Network Manager VPC Attachment.
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
///     let example = vpc_attachment::create(
///         "example",
///         VpcAttachmentArgs::builder()
///             .core_network_id("${exampleAwsccNetworkmanagerCoreNetwork.id}")
///             .subnet_arns(vec!["${exampleAwsSubnet.arn}",])
///             .vpc_arn("${exampleAwsVpc.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_vpc_attachment` using the attachment ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/vpcAttachment:VpcAttachment example attachment-0f8fa60d2238d1bd8
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcAttachmentArgs {
        /// The ID of a core network for the VPC attachment.
        #[builder(into)]
        pub core_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Options for the VPC attachment.
        #[builder(into, default)]
        pub options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkmanager::VpcAttachmentOptions>,
        >,
        /// The subnet ARN of the VPC attachment.
        #[builder(into)]
        pub subnet_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the VPC.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_gestalt_rust::Output<i32>,
        /// The type of attachment.
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// The ARN of a core network.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of a core network for the VPC attachment.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// The Region where the edge is located.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// Options for the VPC attachment.
        pub options: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkmanager::VpcAttachmentOptions>,
        >,
        /// The ID of the attachment account owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The attachment resource ARN.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the segment attachment.
        pub segment_name: pulumi_gestalt_rust::Output<String>,
        /// The state of the attachment.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The subnet ARN of the VPC attachment.
        pub subnet_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the VPC.
        ///
        /// The following arguments are optional:
        pub vpc_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcAttachmentArgs,
    ) -> VpcAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let core_network_id_binding = args.core_network_id.get_output(context);
        let options_binding = args.options.get_output(context);
        let subnet_arns_binding = args.subnet_arns.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_arn_binding = args.vpc_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/vpcAttachment:VpcAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkId".into(),
                    value: &core_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "options".into(),
                    value: &options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetArns".into(),
                    value: &subnet_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcArn".into(),
                    value: &vpc_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcAttachmentResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            attachment_policy_rule_number: o.get_field("attachmentPolicyRuleNumber"),
            attachment_type: o.get_field("attachmentType"),
            core_network_arn: o.get_field("coreNetworkArn"),
            core_network_id: o.get_field("coreNetworkId"),
            edge_location: o.get_field("edgeLocation"),
            options: o.get_field("options"),
            owner_account_id: o.get_field("ownerAccountId"),
            resource_arn: o.get_field("resourceArn"),
            segment_name: o.get_field("segmentName"),
            state: o.get_field("state"),
            subnet_arns: o.get_field("subnetArns"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_arn: o.get_field("vpcArn"),
        }
    }
}
