/// Creates a Redshift authentication profile
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:redshift:AuthenticationProfile
///     properties:
///       authenticationProfileName: example
///       authenticationProfileContent:
///         fn::toJSON:
///           AllowDBUserOverride: '1'
///           Client_ID: ExampleClientID
///           App_ID: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Authentication by `authentication_profile_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/authenticationProfile:AuthenticationProfile test example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod authentication_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticationProfileArgs {
        /// The content of the authentication profile in JSON format. The maximum length of the JSON string is determined by a quota for your account.
        #[builder(into)]
        pub authentication_profile_content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the authentication profile.
        #[builder(into)]
        pub authentication_profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticationProfileResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The content of the authentication profile in JSON format. The maximum length of the JSON string is determined by a quota for your account.
        pub authentication_profile_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the authentication profile.
        pub authentication_profile_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticationProfileArgs,
    ) -> AuthenticationProfileResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticationProfileArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AuthenticationProfileResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticationProfileArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AuthenticationProfileResult {
        let authentication_profile_content_binding = args
            .authentication_profile_content
            .get_output(ctx);
        let authentication_profile_name_binding = args
            .authentication_profile_name
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/authenticationProfile:AuthenticationProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationProfileContent".into(),
                    value: &authentication_profile_content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationProfileName".into(),
                    value: &authentication_profile_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AuthenticationProfileResult {
            id: o.get_id(),
            urn: o.get_urn(),
            authentication_profile_content: o.get_field("authenticationProfileContent"),
            authentication_profile_name: o.get_field("authenticationProfileName"),
        }
    }
}
