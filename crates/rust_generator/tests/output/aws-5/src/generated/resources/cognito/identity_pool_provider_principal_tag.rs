/// Provides an AWS Cognito Identity Principal Mapping.
///
/// ## Import
///
/// Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID and provider name. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag example us-west-2_abc123:CorpAD
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod identity_pool_provider_principal_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolProviderPrincipalTagArgs {
        /// An identity pool ID.
        #[builder(into)]
        pub identity_pool_id: pulumi_gestalt_rust::Input<String>,
        /// The name of the identity provider.
        #[builder(into)]
        pub identity_provider_name: pulumi_gestalt_rust::Input<String>,
        /// String to string map of variables.
        #[builder(into, default)]
        pub principal_tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// use default (username and clientID) attribute mappings.
        #[builder(into, default)]
        pub use_defaults: pulumi_gestalt_rust::Input<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolProviderPrincipalTagResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// An identity pool ID.
        pub identity_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the identity provider.
        pub identity_provider_name: pulumi_gestalt_rust::Output<String>,
        /// String to string map of variables.
        pub principal_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// use default (username and clientID) attribute mappings.
        pub use_defaults: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityPoolProviderPrincipalTagArgs,
    ) -> IdentityPoolProviderPrincipalTagResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityPoolProviderPrincipalTagArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IdentityPoolProviderPrincipalTagResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityPoolProviderPrincipalTagArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IdentityPoolProviderPrincipalTagResult {
        let identity_pool_id_binding = args.identity_pool_id.get_output(ctx);
        let identity_provider_name_binding = args.identity_provider_name.get_output(ctx);
        let principal_tags_binding = args.principal_tags.get_output(ctx);
        let use_defaults_binding = args.use_defaults.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityPoolId".into(),
                    value: &identity_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityProviderName".into(),
                    value: &identity_provider_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalTags".into(),
                    value: &principal_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useDefaults".into(),
                    value: &use_defaults_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IdentityPoolProviderPrincipalTagResult {
            id: o.get_id(),
            urn: o.get_urn(),
            identity_pool_id: o.get_field("identityPoolId"),
            identity_provider_name: o.get_field("identityProviderName"),
            principal_tags: o.get_field("principalTags"),
            use_defaults: o.get_field("useDefaults"),
        }
    }
}
