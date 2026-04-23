/// Provides an Athena Workgroup.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workgroup::create(
///         "example",
///         WorkgroupArgs::builder()
///             .configuration(
///                 WorkgroupConfiguration::builder()
///                     .enforceWorkgroupConfiguration(true)
///                     .publishCloudwatchMetricsEnabled(true)
///                     .resultConfiguration(
///                         WorkgroupConfigurationResultConfiguration::builder()
///                             .encryptionConfiguration(
///                                 WorkgroupConfigurationResultConfigurationEncryptionConfiguration::builder()
///                                     .encryptionOption("SSE_KMS")
///                                     .kmsKeyArn("${exampleAwsKmsKey.arn}")
///                                     .build_struct(),
///                             )
///                             .outputLocation("s3://${exampleAwsS3Bucket.bucket}/output/")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Athena Workgroups using their name. For example:
///
/// ```sh
/// $ pulumi import aws:athena/workgroup:Workgroup example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod workgroup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkgroupArgs {
        /// Configuration block with various settings for the workgroup. Documented below.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::Input<
            Option<super::super::types::athena::WorkgroupConfiguration>,
        >,
        /// Description of the workgroup.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Option to delete the workgroup and its contents even if the workgroup contains any named queries.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Name of the workgroup.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// State of the workgroup. Valid values are `DISABLED` or `ENABLED`. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::Input<Option<String>>,
        /// Key-value map of resource tags for the workgroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkgroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the workgroup
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with various settings for the workgroup. Documented below.
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::athena::WorkgroupConfiguration>,
        >,
        /// Description of the workgroup.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to delete the workgroup and its contents even if the workgroup contains any named queries.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the workgroup.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// State of the workgroup. Valid values are `DISABLED` or `ENABLED`. Defaults to `ENABLED`.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the workgroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkgroupArgs,
    ) -> WorkgroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkgroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> WorkgroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkgroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> WorkgroupResult {
        let configuration_binding = args.configuration.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let force_destroy_binding = args.force_destroy.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let state_binding = args.state.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:athena/workgroup:Workgroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        WorkgroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            configuration: o.get_field("configuration"),
            description: o.get_field("description"),
            force_destroy: o.get_field("forceDestroy"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
