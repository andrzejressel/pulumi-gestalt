#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod module_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct moduleTestArgs {
        #[builder(into, default)]
        pub mod1: pulumi_gestalt_rust::Input<Option<super::types::mod1::Typ>>,
        #[builder(into, default)]
        pub val: pulumi_gestalt_rust::Input<Option<super::types::Typ>>,
    }
    #[allow(dead_code)]
    pub struct moduleTestResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: moduleTestArgs,
    ) -> moduleTestResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: moduleTestArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> moduleTestResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: moduleTestArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> moduleTestResult {
        let mod1_binding = args.mod1.get_output(ctx);
        let val_binding = args.val.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:moduleTest".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mod1".into(),
                    value: &mod1_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "val".into(),
                    value: &val_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        moduleTestResult {
            id: o.get_id(),
            urn: o.get_urn(),
        }
    }
}
