/// Provides a Cognito Resource Server.
///
/// ## Example Usage
///
/// ### Create a basic resource server
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("pool").build_struct(),
///     );
///     let resource = resource_server::create(
///         "resource",
///         ResourceServerArgs::builder()
///             .identifier("https://example.com")
///             .name("example")
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create a resource server with sample-scope
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("pool").build_struct(),
///     );
///     let resource = resource_server::create(
///         "resource",
///         ResourceServerArgs::builder()
///             .identifier("https://example.com")
///             .name("example")
///             .scopes(
///                 vec![
///                     ResourceServerScope::builder()
///                     .scopeDescription("a Sample Scope Description")
///                     .scopeName("sample-scope").build_struct(),
///                 ],
///             )
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cognito_resource_server` using their User Pool ID and Identifier. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/resourceServer:ResourceServer example "us-west-2_abc123|https://example.com"
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resource_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceServerArgs {
        /// An identifier for the resource server.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::Input<String>,
        /// A name for the resource server.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A list of Authorization Scope.
        #[builder(into, default)]
        pub scopes: pulumi_gestalt_rust::Input<
            Option<Vec<super::super::types::cognito::ResourceServerScope>>,
        >,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceServerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// An identifier for the resource server.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// A name for the resource server.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of all scopes configured for this resource server in the format identifier/scope_name.
        pub scope_identifiers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of Authorization Scope.
        pub scopes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cognito::ResourceServerScope>>,
        >,
        /// User pool the client belongs to.
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceServerArgs,
    ) -> ResourceServerResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceServerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ResourceServerResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceServerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ResourceServerResult {
        let identifier_binding = args.identifier.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let scopes_binding = args.scopes.get_output(ctx);
        let user_pool_id_binding = args.user_pool_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/resourceServer:ResourceServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ResourceServerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            identifier: o.get_field("identifier"),
            name: o.get_field("name"),
            scope_identifiers: o.get_field("scopeIdentifiers"),
            scopes: o.get_field("scopes"),
            user_pool_id: o.get_field("userPoolId"),
        }
    }
}
