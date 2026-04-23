/// ## Example Usage
///
/// Example creating a RuntimeConfig resource.
///
/// ```yaml
/// resources:
///   my-runtime-config:
///     type: gcp:runtimeconfig:Config
///     properties:
///       name: my-service-runtime-config
///       description: Runtime configuration values for my service
/// ```
///
/// ## Import
///
/// Runtime Configs can be imported using the `name` or full config name, e.g.
///
/// * `projects/{{project_id}}/configs/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Runtime Configs can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:runtimeconfig/config:Config default projects/{{project_id}}/configs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:runtimeconfig/config:Config default {{name}}
/// ```
///
/// When importing using only the name, the provider project must be set.
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigArgs {
        /// The description to associate with the runtime
        /// config.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the runtime config.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The description to associate with the runtime
        /// config.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the runtime config.
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigArgs,
    ) -> ConfigResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ConfigResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ConfigResult {
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:runtimeconfig/config:Config".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ConfigResult {
            id: o.get_id(),
            urn: o.get_urn(),
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
