#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod type_ {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TypeArgs {
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::Input<
            Option<super::super::super::super::types::impl_::let_::loop_::Type>,
        >,
    }
    #[allow(dead_code)]
    pub struct TypeResult {
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
        args: TypeArgs,
    ) -> TypeResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TypeArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TypeResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TypeArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TypeResult {
        let type__binding = args.type_.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:impl/let/loop:Type".into(),
            name: name.to_string(),
            version: super::super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TypeResult {
            id: o.get_id(),
            urn: o.get_urn(),
        }
    }
}
