/// Provides a Cognito User Pool Client resource.
///
/// To manage a User Pool Client created by another service, such as when [configuring an OpenSearch Domain to use Cognito authentication](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html),
/// use the `aws.cognito.ManagedUserPoolClient` resource instead.
///
/// ## Example Usage
///
/// ### Create a basic user pool client
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let client = user_pool_client::create(
///         "client",
///         UserPoolClientArgs::builder()
///             .name("client")
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("pool").build_struct(),
///     );
/// }
/// ```
///
/// ### Create a user pool client with no SRP authentication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let client = user_pool_client::create(
///         "client",
///         UserPoolClientArgs::builder()
///             .explicit_auth_flows(vec!["ADMIN_NO_SRP_AUTH",])
///             .generate_secret(true)
///             .name("client")
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("pool").build_struct(),
///     );
/// }
/// ```
///
/// ### Create a user pool client with pinpoint analytics
///
/// ```yaml
/// resources:
///   testUserPoolClient:
///     type: aws:cognito:UserPoolClient
///     name: test
///     properties:
///       name: pool_client
///       userPoolId: ${testUserPool.id}
///       analyticsConfiguration:
///         applicationId: ${testApp.applicationId}
///         externalId: some_id
///         roleArn: ${testRole.arn}
///         userDataShared: true
///   testUserPool:
///     type: aws:cognito:UserPool
///     name: test
///     properties:
///       name: pool
///   testApp:
///     type: aws:pinpoint:App
///     name: test
///     properties:
///       name: pinpoint
///   testRole:
///     type: aws:iam:Role
///     name: test
///     properties:
///       name: role
///       assumeRolePolicy: ${assumeRole.json}
///   testRolePolicy:
///     type: aws:iam:RolePolicy
///     name: test
///     properties:
///       name: role_policy
///       role: ${testRole.id}
///       policy: ${test.json}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - cognito-idp.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - mobiletargeting:UpdateEndpoint
///               - mobiletargeting:PutEvents
///             resources:
///               - arn:aws:mobiletargeting:*:${current.accountId}:apps/${testApp.applicationId}*
/// ```
///
/// ### Create a user pool client with Cognito as the identity provider
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
///     let userpoolClient = user_pool_client::create(
///         "userpoolClient",
///         UserPoolClientArgs::builder()
///             .allowed_oauth_flows(vec!["code", "implicit",])
///             .allowed_oauth_flows_user_pool_client(true)
///             .allowed_oauth_scopes(vec!["email", "openid",])
///             .callback_urls(vec!["https://example.com",])
///             .name("client")
///             .supported_identity_providers(vec!["COGNITO",])
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User Pool Clients using the `id` of the Cognito User Pool, and the `id` of the Cognito User Pool Client. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/userPoolClient:UserPoolClient client us-west-2_abc123/3ho4ek12345678909nh3fmhpko
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_pool_client {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoolClientArgs {
        /// Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
        #[builder(into, default)]
        pub access_token_validity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub allowed_oauth_flows: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
        #[builder(into, default)]
        pub allowed_oauth_flows_user_pool_client: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub allowed_oauth_scopes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Configuration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
        #[builder(into, default)]
        pub analytics_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolClientAnalyticsConfiguration>,
        >,
        /// Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
        #[builder(into, default)]
        pub auth_session_validity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub callback_urls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Default redirect URI and must be included in the list of callback URLs.
        #[builder(into, default)]
        pub default_redirect_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables the propagation of additional user context data.
        #[builder(into, default)]
        pub enable_propagate_additional_user_context_data: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enables or disables token revocation.
        #[builder(into, default)]
        pub enable_token_revocation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
        #[builder(into, default)]
        pub explicit_auth_flows: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Boolean flag indicating whether an application secret should be generated.
        #[builder(into, default)]
        pub generate_secret: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
        #[builder(into, default)]
        pub id_token_validity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub logout_urls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the application client.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
        #[builder(into, default)]
        pub prevent_user_existence_errors: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// List of user pool attributes that the application client can read from.
        #[builder(into, default)]
        pub read_attributes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
        #[builder(into, default)]
        pub refresh_token_validity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
        #[builder(into, default)]
        pub supported_identity_providers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Configuration block for representing the validity times in units. See details below. Detailed below.
        #[builder(into, default)]
        pub token_validity_units: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolClientTokenValidityUnits>,
        >,
        /// User pool the client belongs to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of user pool attributes that the application client can write to.
        #[builder(into, default)]
        pub write_attributes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct UserPoolClientResult {
        /// Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
        pub access_token_validity: pulumi_gestalt_rust::Output<i32>,
        /// List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub allowed_oauth_flows: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
        pub allowed_oauth_flows_user_pool_client: pulumi_gestalt_rust::Output<bool>,
        /// List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub allowed_oauth_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Configuration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
        pub analytics_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolClientAnalyticsConfiguration>,
        >,
        /// Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
        pub auth_session_validity: pulumi_gestalt_rust::Output<i32>,
        /// List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub callback_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Client secret of the user pool client.
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// Default redirect URI and must be included in the list of callback URLs.
        pub default_redirect_uri: pulumi_gestalt_rust::Output<String>,
        /// Enables the propagation of additional user context data.
        pub enable_propagate_additional_user_context_data: pulumi_gestalt_rust::Output<
            bool,
        >,
        /// Enables or disables token revocation.
        pub enable_token_revocation: pulumi_gestalt_rust::Output<bool>,
        /// List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
        pub explicit_auth_flows: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Boolean flag indicating whether an application secret should be generated.
        pub generate_secret: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
        pub id_token_validity: pulumi_gestalt_rust::Output<i32>,
        /// List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub logout_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the application client.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
        pub prevent_user_existence_errors: pulumi_gestalt_rust::Output<String>,
        /// List of user pool attributes that the application client can read from.
        pub read_attributes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
        pub refresh_token_validity: pulumi_gestalt_rust::Output<i32>,
        /// List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
        pub supported_identity_providers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Configuration block for representing the validity times in units. See details below. Detailed below.
        pub token_validity_units: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolClientTokenValidityUnits>,
        >,
        /// User pool the client belongs to.
        ///
        /// The following arguments are optional:
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
        /// List of user pool attributes that the application client can write to.
        pub write_attributes: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPoolClientArgs,
    ) -> UserPoolClientResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_token_validity_binding = args
            .access_token_validity
            .get_output(context);
        let allowed_oauth_flows_binding = args.allowed_oauth_flows.get_output(context);
        let allowed_oauth_flows_user_pool_client_binding = args
            .allowed_oauth_flows_user_pool_client
            .get_output(context);
        let allowed_oauth_scopes_binding = args.allowed_oauth_scopes.get_output(context);
        let analytics_configuration_binding = args
            .analytics_configuration
            .get_output(context);
        let auth_session_validity_binding = args
            .auth_session_validity
            .get_output(context);
        let callback_urls_binding = args.callback_urls.get_output(context);
        let default_redirect_uri_binding = args.default_redirect_uri.get_output(context);
        let enable_propagate_additional_user_context_data_binding = args
            .enable_propagate_additional_user_context_data
            .get_output(context);
        let enable_token_revocation_binding = args
            .enable_token_revocation
            .get_output(context);
        let explicit_auth_flows_binding = args.explicit_auth_flows.get_output(context);
        let generate_secret_binding = args.generate_secret.get_output(context);
        let id_token_validity_binding = args.id_token_validity.get_output(context);
        let logout_urls_binding = args.logout_urls.get_output(context);
        let name_binding = args.name.get_output(context);
        let prevent_user_existence_errors_binding = args
            .prevent_user_existence_errors
            .get_output(context);
        let read_attributes_binding = args.read_attributes.get_output(context);
        let refresh_token_validity_binding = args
            .refresh_token_validity
            .get_output(context);
        let supported_identity_providers_binding = args
            .supported_identity_providers
            .get_output(context);
        let token_validity_units_binding = args.token_validity_units.get_output(context);
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let write_attributes_binding = args.write_attributes.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/userPoolClient:UserPoolClient".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessTokenValidity".into(),
                    value: &access_token_validity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedOauthFlows".into(),
                    value: &allowed_oauth_flows_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedOauthFlowsUserPoolClient".into(),
                    value: &allowed_oauth_flows_user_pool_client_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedOauthScopes".into(),
                    value: &allowed_oauth_scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyticsConfiguration".into(),
                    value: &analytics_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authSessionValidity".into(),
                    value: &auth_session_validity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "callbackUrls".into(),
                    value: &callback_urls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRedirectUri".into(),
                    value: &default_redirect_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablePropagateAdditionalUserContextData".into(),
                    value: &enable_propagate_additional_user_context_data_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableTokenRevocation".into(),
                    value: &enable_token_revocation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "explicitAuthFlows".into(),
                    value: &explicit_auth_flows_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "generateSecret".into(),
                    value: &generate_secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idTokenValidity".into(),
                    value: &id_token_validity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logoutUrls".into(),
                    value: &logout_urls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preventUserExistenceErrors".into(),
                    value: &prevent_user_existence_errors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readAttributes".into(),
                    value: &read_attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "refreshTokenValidity".into(),
                    value: &refresh_token_validity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedIdentityProviders".into(),
                    value: &supported_identity_providers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenValidityUnits".into(),
                    value: &token_validity_units_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "writeAttributes".into(),
                    value: &write_attributes_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserPoolClientResult {
            access_token_validity: o.get_field("accessTokenValidity"),
            allowed_oauth_flows: o.get_field("allowedOauthFlows"),
            allowed_oauth_flows_user_pool_client: o
                .get_field("allowedOauthFlowsUserPoolClient"),
            allowed_oauth_scopes: o.get_field("allowedOauthScopes"),
            analytics_configuration: o.get_field("analyticsConfiguration"),
            auth_session_validity: o.get_field("authSessionValidity"),
            callback_urls: o.get_field("callbackUrls"),
            client_secret: o.get_field("clientSecret"),
            default_redirect_uri: o.get_field("defaultRedirectUri"),
            enable_propagate_additional_user_context_data: o
                .get_field("enablePropagateAdditionalUserContextData"),
            enable_token_revocation: o.get_field("enableTokenRevocation"),
            explicit_auth_flows: o.get_field("explicitAuthFlows"),
            generate_secret: o.get_field("generateSecret"),
            id_token_validity: o.get_field("idTokenValidity"),
            logout_urls: o.get_field("logoutUrls"),
            name: o.get_field("name"),
            prevent_user_existence_errors: o.get_field("preventUserExistenceErrors"),
            read_attributes: o.get_field("readAttributes"),
            refresh_token_validity: o.get_field("refreshTokenValidity"),
            supported_identity_providers: o.get_field("supportedIdentityProviders"),
            token_validity_units: o.get_field("tokenValidityUnits"),
            user_pool_id: o.get_field("userPoolId"),
            write_attributes: o.get_field("writeAttributes"),
        }
    }
}
