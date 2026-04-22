/// Provides a resource which manages Cloudflare API tokens.
///
/// Read more about permission groups and their applicable scopes in the
/// [developer documentation](https://developers.cloudflare.com/api/tokens/create/permissions).
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod api_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiTokenArgs {
        /// Conditions under which the token should be considered valid.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::Input<
            Option<super::types::ApiTokenCondition>,
        >,
        /// The expiration time on or after which the token MUST NOT be accepted for processing.
        #[builder(into, default)]
        pub expires_on: pulumi_gestalt_rust::Input<Option<String>>,
        /// Name of the API Token.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
        /// The time before which the token MUST NOT be accepted for processing.
        #[builder(into, default)]
        pub not_before: pulumi_gestalt_rust::Input<Option<String>>,
        /// Permissions policy. Multiple policy blocks can be defined.
        #[builder(into)]
        pub policies: pulumi_gestalt_rust::Input<Vec<super::types::ApiTokenPolicy>>,
    }
    #[allow(dead_code)]
    pub struct ApiTokenResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Conditions under which the token should be considered valid.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::types::ApiTokenCondition>,
        >,
        /// The expiration time on or after which the token MUST NOT be accepted for processing.
        pub expires_on: pulumi_gestalt_rust::Output<Option<String>>,
        /// Timestamp of when the token was issued.
        pub issued_on: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the token was last modified.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// Name of the API Token.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The time before which the token MUST NOT be accepted for processing.
        pub not_before: pulumi_gestalt_rust::Output<Option<String>>,
        /// Permissions policy. Multiple policy blocks can be defined.
        pub policies: pulumi_gestalt_rust::Output<Vec<super::types::ApiTokenPolicy>>,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The value of the API Token.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiTokenArgs,
    ) -> ApiTokenResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiTokenArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ApiTokenResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiTokenArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ApiTokenResult {
        let condition_binding = args.condition.get_output(ctx);
        let expires_on_binding = args.expires_on.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let not_before_binding = args.not_before.get_output(ctx);
        let policies_binding = args.policies.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiToken:ApiToken".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiresOn".into(),
                    value: &expires_on_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notBefore".into(),
                    value: &not_before_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policies".into(),
                    value: &policies_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ApiTokenResult {
            id: o.get_id(),
            urn: o.get_urn(),
            condition: o.get_field("condition"),
            expires_on: o.get_field("expiresOn"),
            issued_on: o.get_field("issuedOn"),
            modified_on: o.get_field("modifiedOn"),
            name: o.get_field("name"),
            not_before: o.get_field("notBefore"),
            policies: o.get_field("policies"),
            status: o.get_field("status"),
            value: o.get_field("value"),
        }
    }
}
