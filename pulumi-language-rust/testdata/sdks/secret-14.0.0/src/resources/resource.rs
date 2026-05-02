#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        #[builder(into)]
        pub private: pulumi_gestalt_rust::Input<String>,
        #[builder(into)]
        pub private_data: pulumi_gestalt_rust::Input<super::types::Data>,
        #[builder(into)]
        pub public: pulumi_gestalt_rust::Input<String>,
        #[builder(into)]
        pub public_data: pulumi_gestalt_rust::Input<super::types::Data>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub private: pulumi_gestalt_rust::Output<String>,
        pub private_data: pulumi_gestalt_rust::Output<super::types::Data>,
        pub public: pulumi_gestalt_rust::Output<String>,
        pub public_data: pulumi_gestalt_rust::Output<super::types::Data>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ResourceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ResourceResult {
        let private_binding = args.private.get_output(ctx);
        let private_data_binding = args.private_data.get_output(ctx);
        let public_binding = args.public.get_output(ctx);
        let public_data_binding = args.public_data.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "secret:index:Resource".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "private".into(),
                    value: &private_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateData".into(),
                    value: &private_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "public".into(),
                    value: &public_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicData".into(),
                    value: &public_data_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ResourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            private: o.get_field("private"),
            private_data: o.get_field("privateData"),
            public: o.get_field("public"),
            public_data: o.get_field("publicData"),
        }
    }
}
