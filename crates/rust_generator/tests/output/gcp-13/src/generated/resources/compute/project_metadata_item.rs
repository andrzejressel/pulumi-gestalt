/// Manages a single key/value pair on metadata common to all instances for
/// a project in GCE. Using `gcp.compute.ProjectMetadataItem` lets you
/// manage a single key/value setting in the provider rather than the entire
/// project metadata map.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = project_metadata_item::create(
///         "default",
///         ProjectMetadataItemArgs::builder()
///             .key("my_metadata")
///             .value("my_value")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Project metadata items can be imported using the `key`, e.g.
///
/// * `{{key}}`
///
/// When using the `pulumi import` command, project metadata items can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/projectMetadataItem:ProjectMetadataItem default {{key}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod project_metadata_item {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectMetadataItemArgs {
        /// The metadata key to set.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::Input<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// The value to set for the given metadata key.
        ///
        /// - - -
        #[builder(into)]
        pub value: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ProjectMetadataItemResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The metadata key to set.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The value to set for the given metadata key.
        ///
        /// - - -
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectMetadataItemArgs,
    ) -> ProjectMetadataItemResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectMetadataItemArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ProjectMetadataItemResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectMetadataItemArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ProjectMetadataItemResult {
        let key_binding = args.key.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let value_binding = args.value.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/projectMetadataItem:ProjectMetadataItem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ProjectMetadataItemResult {
            id: o.get_id(),
            urn: o.get_urn(),
            key: o.get_field("key"),
            project: o.get_field("project"),
            value: o.get_field("value"),
        }
    }
}
