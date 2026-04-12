/// Provides the ability to register a target with an AWS VPC Lattice Target Group.
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
///     let example = target_group_attachment::create(
///         "example",
///         TargetGroupAttachmentArgs::builder()
///             .target(
///                 TargetGroupAttachmentTarget::builder()
///                     .id("${exampleAwsLb.arn}")
///                     .port(80)
///                     .build_struct(),
///             )
///             .target_group_identifier("${exampleAwsVpclatticeTargetGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod target_group_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentArgs {
        /// The target.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::vpclattice::TargetGroupAttachmentTarget,
        >,
        /// The ID or Amazon Resource Name (ARN) of the target group.
        #[builder(into)]
        pub target_group_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The target.
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::vpclattice::TargetGroupAttachmentTarget,
        >,
        /// The ID or Amazon Resource Name (ARN) of the target group.
        pub target_group_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetGroupAttachmentArgs,
    ) -> TargetGroupAttachmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetGroupAttachmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TargetGroupAttachmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetGroupAttachmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TargetGroupAttachmentResult {
        let target_binding = args.target.get_output(ctx);
        let target_group_identifier_binding = args
            .target_group_identifier
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/targetGroupAttachment:TargetGroupAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetGroupIdentifier".into(),
                    value: &target_group_identifier_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TargetGroupAttachmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            target: o.get_field("target"),
            target_group_identifier: o.get_field("targetGroupIdentifier"),
        }
    }
}
