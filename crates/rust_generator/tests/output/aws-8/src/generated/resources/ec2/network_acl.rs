/// Provides an network ACL resource. You might set up network ACLs with rules similar
/// to your security groups in order to add an additional layer of security to your VPC.
///
/// > **NOTE on Network ACLs and Network ACL Rules:** This provider currently
/// provides both a standalone Network ACL Rule resource and a Network ACL resource with rules
/// defined in-line. At this time you cannot use a Network ACL with in-line rules
/// in conjunction with any Network ACL Rule resources. Doing so will cause
/// a conflict of rule settings and will overwrite rules.
///
/// > **NOTE on Network ACLs and Network ACL Associations:** the provider provides both a standalone network ACL association
/// resource and a network ACL resource with a `subnet_ids` attribute. Do not use the same subnet ID in both a network ACL
/// resource and a network ACL association resource. Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:NetworkAcl
///     properties:
///       vpcId: ${mainAwsVpc.id}
///       egress:
///         - protocol: tcp
///           ruleNo: 200
///           action: allow
///           cidrBlock: 10.3.0.0/18
///           fromPort: 443
///           toPort: 443
///       ingress:
///         - protocol: tcp
///           ruleNo: 100
///           action: allow
///           cidrBlock: 10.3.0.0/18
///           fromPort: 80
///           toPort: 80
///       tags:
///         Name: main
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network ACLs using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkAcl:NetworkAcl main acl-7aaabd18
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAclArgs {
        /// Specifies an egress rule. Parameters defined below.
        #[builder(into, default)]
        pub egress: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::NetworkAclEgress>>,
        >,
        /// Specifies an ingress rule. Parameters defined below.
        #[builder(into, default)]
        pub ingress: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::NetworkAclIngress>>,
        >,
        /// A list of Subnet IDs to apply the ACL to
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the associated VPC.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkAclResult {
        /// The ARN of the network ACL
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies an egress rule. Parameters defined below.
        pub egress: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkAclEgress>,
        >,
        /// Specifies an ingress rule. Parameters defined below.
        pub ingress: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkAclIngress>,
        >,
        /// The ID of the AWS account that owns the network ACL.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Subnet IDs to apply the ACL to
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the associated VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkAclArgs,
    ) -> NetworkAclResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let egress_binding = args.egress.get_output(context);
        let ingress_binding = args.ingress.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/networkAcl:NetworkAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "egress".into(),
                    value: &egress_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkAclResult {
            arn: o.get_field("arn"),
            egress: o.get_field("egress"),
            ingress: o.get_field("ingress"),
            owner_id: o.get_field("ownerId"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
