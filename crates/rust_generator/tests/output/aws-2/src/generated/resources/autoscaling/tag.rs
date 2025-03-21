/// Manages an individual Autoscaling Group (ASG) tag. This resource should only be used in cases where ASGs are created outside the provider (e.g., ASGs implicitly created by EKS Node Groups).
///
/// > **NOTE:** This tagging resource should not be combined with the resource for managing the parent resource. For example, using `aws.autoscaling.Group` and `aws.autoscaling.Tag` to manage tags of the same ASG will cause a perpetual difference where the `aws.autoscaling.Group` resource will try to remove the tag being added by the `aws.autoscaling.Tag` resource.
///
/// > **NOTE:** This tagging resource does not use the provider `ignore_tags` configuration.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_autoscaling_group_tag` using the ASG name and key, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:autoscaling/tag:Tag example asg-example,k8s.io/cluster-autoscaler/node-template/label/eks.amazonaws.com/capacityType
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Name of the Autoscaling Group to apply the tag to.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tag to create. The `tag` block is documented below.
        #[builder(into)]
        pub tag: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::autoscaling::TagTag,
        >,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// Name of the Autoscaling Group to apply the tag to.
        pub autoscaling_group_name: pulumi_gestalt_rust::Output<String>,
        /// Tag to create. The `tag` block is documented below.
        pub tag: pulumi_gestalt_rust::Output<super::super::types::autoscaling::TagTag>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagArgs,
    ) -> TagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_group_name_binding = args
            .autoscaling_group_name
            .get_output(context);
        let tag_binding = args.tag.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:autoscaling/tag:Tag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tag".into(),
                    value: &tag_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagResult {
            autoscaling_group_name: o.get_field("autoscalingGroupName"),
            tag: o.get_field("tag"),
        }
    }
}
