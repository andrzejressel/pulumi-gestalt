#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod credential_user_managed_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CredentialUserManagedIdentityArgs {
        /// (Optional) List of string annotations.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// The resource ID of the parent Data Factory
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::Input<String>,
        /// (Optional) Short text description
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The resource ID of the User Assigned Managed Identity
        #[builder(into)]
        pub identity_id: pulumi_gestalt_rust::Input<String>,
        /// The desired name of the credential resource
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CredentialUserManagedIdentityResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// (Optional) List of string annotations.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The resource ID of the parent Data Factory
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// (Optional) Short text description
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the User Assigned Managed Identity
        pub identity_id: pulumi_gestalt_rust::Output<String>,
        /// The desired name of the credential resource
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialUserManagedIdentityArgs,
    ) -> CredentialUserManagedIdentityResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialUserManagedIdentityArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CredentialUserManagedIdentityResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialUserManagedIdentityArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CredentialUserManagedIdentityResult {
        let annotations_binding = args.annotations.get_output(ctx);
        let data_factory_id_binding = args.data_factory_id.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let identity_id_binding = args.identity_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/credentialUserManagedIdentity:CredentialUserManagedIdentity"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CredentialUserManagedIdentityResult {
            id: o.get_id(),
            urn: o.get_urn(),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            identity_id: o.get_field("identityId"),
            name: o.get_field("name"),
        }
    }
}
