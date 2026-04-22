#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resource_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Resource2Args {
        #[builder(into, default)]
        pub common_type: pulumi_gestalt_rust::Input<
            Option<super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type2: pulumi_gestalt_rust::Input<Option<super::super::types::ns2::Type2>>,
    }
    #[allow(dead_code)]
    pub struct Resource2Result {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub common_type: pulumi_gestalt_rust::Output<
            Option<super::super::types::common::CommonType>,
        >,
        pub type2: pulumi_gestalt_rust::Output<Option<super::super::types::ns2::Type2>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Resource2Args,
    ) -> Resource2Result {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Resource2Args,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> Resource2Result {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Resource2Args,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> Resource2Result {
        let common_type_binding = args.common_type.get_output(ctx);
        let type2_binding = args.type2.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:ns2:Resource2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commonType".into(),
                    value: &common_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type2".into(),
                    value: &type2_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        Resource2Result {
            id: o.get_id(),
            urn: o.get_urn(),
            common_type: o.get_field("commonType"),
            type2: o.get_field("type2"),
        }
    }
}
