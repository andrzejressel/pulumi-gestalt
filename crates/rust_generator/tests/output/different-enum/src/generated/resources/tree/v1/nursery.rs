#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod nursery {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NurseryArgs {
        /// The sizes of trees available
        #[builder(into, default)]
        pub sizes: pulumi_gestalt_rust::InputOrOutput<
            Option<
                std::collections::HashMap<
                    String,
                    super::super::super::types::tree::v1::TreeSize,
                >,
            >,
        >,
        /// The varieties available
        #[builder(into)]
        pub varieties: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::tree::v1::RubberTreeVariety>,
        >,
    }
    #[allow(dead_code)]
    pub struct NurseryResult {
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
        args: NurseryArgs,
    ) -> NurseryResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NurseryArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> NurseryResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NurseryArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> NurseryResult {
        let sizes_binding = args.sizes.get_output(ctx);
        let varieties_binding = args.varieties.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "plant:tree/v1:Nursery".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sizes".into(),
                    value: &sizes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "varieties".into(),
                    value: &varieties_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        NurseryResult {
            id: o.get_id(),
            urn: o.get_urn(),
        }
    }
}
