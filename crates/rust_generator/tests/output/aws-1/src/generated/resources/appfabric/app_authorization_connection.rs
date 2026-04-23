/// Resource for managing an AWS AppFabric App Authorization Connection.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app_authorization_connection::create(
///         "example",
///         AppAuthorizationConnectionArgs::builder()
///             .app_authorization_arn("${test.arn}")
///             .app_bundle_arn("${arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod app_authorization_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppAuthorizationConnectionArgs {
        /// The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.
        #[builder(into)]
        pub app_authorization_arn: pulumi_gestalt_rust::Input<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        #[builder(into)]
        pub app_bundle_arn: pulumi_gestalt_rust::Input<String>,
        /// Contains OAuth2 authorization information.This is required if the app authorization for the request is configured with an OAuth2 (oauth2) authorization type.
        #[builder(into, default)]
        pub auth_request: pulumi_gestalt_rust::Input<
            Option<super::super::types::appfabric::AppAuthorizationConnectionAuthRequest>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::appfabric::AppAuthorizationConnectionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppAuthorizationConnectionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the application.
        pub app: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.
        pub app_authorization_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        pub app_bundle_arn: pulumi_gestalt_rust::Output<String>,
        /// Contains OAuth2 authorization information.This is required if the app authorization for the request is configured with an OAuth2 (oauth2) authorization type.
        pub auth_request: pulumi_gestalt_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationConnectionAuthRequest>,
        >,
        /// Contains information about an application tenant, such as the application display name and identifier.
        pub tenants: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appfabric::AppAuthorizationConnectionTenant>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationConnectionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppAuthorizationConnectionArgs,
    ) -> AppAuthorizationConnectionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppAuthorizationConnectionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AppAuthorizationConnectionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppAuthorizationConnectionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AppAuthorizationConnectionResult {
        let app_authorization_arn_binding = args.app_authorization_arn.get_output(ctx);
        let app_bundle_arn_binding = args.app_bundle_arn.get_output(ctx);
        let auth_request_binding = args.auth_request.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appfabric/appAuthorizationConnection:AppAuthorizationConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appAuthorizationArn".into(),
                    value: &app_authorization_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appBundleArn".into(),
                    value: &app_bundle_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authRequest".into(),
                    value: &auth_request_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AppAuthorizationConnectionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            app: o.get_field("app"),
            app_authorization_arn: o.get_field("appAuthorizationArn"),
            app_bundle_arn: o.get_field("appBundleArn"),
            auth_request: o.get_field("authRequest"),
            tenants: o.get_field("tenants"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
