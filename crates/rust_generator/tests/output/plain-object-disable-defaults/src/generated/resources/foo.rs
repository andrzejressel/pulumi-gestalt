/// test new feature with resoruces
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod foo {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FooArgs {
        #[builder(into, default)]
        pub argument: pulumi_gestalt_rust::Input<Option<String>>,
        /// Options for tuning the Kubernetes client used by a Provider.
        #[builder(into)]
        pub backup_kube_client_settings: pulumi_gestalt_rust::Input<
            super::types::KubeClientSettings,
        >,
        /// Options for tuning the Kubernetes client used by a Provider.
        #[builder(into, default)]
        pub kube_client_settings: pulumi_gestalt_rust::Input<
            Option<super::types::KubeClientSettings>,
        >,
        /// describing things
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::Input<
            Option<super::types::LayeredType>,
        >,
    }
    #[allow(dead_code)]
    pub struct FooResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A test for plain types
        pub default_kube_client_settings: pulumi_gestalt_rust::Output<
            Option<super::types::KubeClientSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FooArgs,
    ) -> FooResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FooArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FooResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FooArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FooResult {
        let argument_binding = args.argument.get_output(ctx);
        let backup_kube_client_settings_binding = args
            .backup_kube_client_settings
            .get_output(ctx);
        let kube_client_settings_binding = args.kube_client_settings.get_output(ctx);
        let settings_binding = args.settings.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:Foo".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "argument".into(),
                    value: &argument_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupKubeClientSettings".into(),
                    value: &backup_kube_client_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubeClientSettings".into(),
                    value: &kube_client_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: &settings_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FooResult {
            id: o.get_id(),
            urn: o.get_urn(),
            default_kube_client_settings: o.get_field("defaultKubeClientSettings"),
        }
    }
}
