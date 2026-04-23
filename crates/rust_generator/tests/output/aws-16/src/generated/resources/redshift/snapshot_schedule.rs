/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = snapshot_schedule::create(
///         "default",
///         SnapshotScheduleArgs::builder()
///             .definitions(vec!["rate(12 hours)",])
///             .identifier("tf-redshift-snapshot-schedule")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Snapshot Schedule using the `identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/snapshotSchedule:SnapshotSchedule default tf-redshift-snapshot-schedule
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod snapshot_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotScheduleArgs {
        /// The definition of the snapshot schedule. The definition is made up of schedule expressions, for example `cron(30 12 *)` or `rate(12 hours)`.
        #[builder(into)]
        pub definitions: pulumi_gestalt_rust::Input<Vec<String>>,
        /// The description of the snapshot schedule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Whether to destroy all associated clusters with this snapshot schedule on deletion. Must be enabled and applied before attempting deletion.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The snapshot schedule identifier. If omitted, this provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub identifier: pulumi_gestalt_rust::Input<Option<String>>,
        /// Creates a unique
        /// identifier beginning with the specified prefix. Conflicts with `identifier`.
        #[builder(into, default)]
        pub identifier_prefix: pulumi_gestalt_rust::Input<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SnapshotScheduleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Redshift Snapshot Schedule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The definition of the snapshot schedule. The definition is made up of schedule expressions, for example `cron(30 12 *)` or `rate(12 hours)`.
        pub definitions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The description of the snapshot schedule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to destroy all associated clusters with this snapshot schedule on deletion. Must be enabled and applied before attempting deletion.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The snapshot schedule identifier. If omitted, this provider will assign a random, unique identifier.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique
        /// identifier beginning with the specified prefix. Conflicts with `identifier`.
        pub identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: SnapshotScheduleArgs,
    ) -> SnapshotScheduleResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotScheduleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SnapshotScheduleResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotScheduleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SnapshotScheduleResult {
        let definitions_binding = args.definitions.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let force_destroy_binding = args.force_destroy.get_output(ctx);
        let identifier_binding = args.identifier.get_output(ctx);
        let identifier_prefix_binding = args.identifier_prefix.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/snapshotSchedule:SnapshotSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definitions".into(),
                    value: &definitions_binding.drop_type(),
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
                    name: "identifier".into(),
                    value: &identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifierPrefix".into(),
                    value: &identifier_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SnapshotScheduleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            definitions: o.get_field("definitions"),
            description: o.get_field("description"),
            force_destroy: o.get_field("forceDestroy"),
            identifier: o.get_field("identifier"),
            identifier_prefix: o.get_field("identifierPrefix"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
