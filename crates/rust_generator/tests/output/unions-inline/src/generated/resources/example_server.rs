#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod example_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleServerArgs {
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<
                pulumi_gestalt_rust::OneOf2<
                    super::types::ServerPropertiesForReplica,
                    super::types::ServerPropertiesForRestore,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleServerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleServerArgs,
    ) -> ExampleServerResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleServerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ExampleServerResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleServerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ExampleServerResult {
        let properties_binding = args.properties.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: &properties_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ExampleServerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
        }
    }
}
