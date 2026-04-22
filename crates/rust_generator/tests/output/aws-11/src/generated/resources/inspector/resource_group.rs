/// Provides an Amazon Inspector Classic Resource Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:inspector:ResourceGroup
///     properties:
///       tags:
///         Name: foo
///         Env: bar
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resource_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupArgs {
        /// Key-value map of tags that are used to select the EC2 instances to be included in an Amazon Inspector assessment target.
        #[builder(into)]
        pub tags: pulumi_gestalt_rust::Input<std::collections::HashMap<String, String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The resource group ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags that are used to select the EC2 instances to be included in an Amazon Inspector assessment target.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceGroupArgs,
    ) -> ResourceGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ResourceGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ResourceGroupResult {
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector/resourceGroup:ResourceGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ResourceGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            tags: o.get_field("tags"),
        }
    }
}
