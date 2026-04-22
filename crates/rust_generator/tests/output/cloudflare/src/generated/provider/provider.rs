/// The provider type for the cloudflare package. By default, resources use package-wide configuration
/// settings, however an explicit `Provider` instance may be created and passed during resource
/// construction to achieve fine-grained programmatic control over provider settings. See the
/// [documentation](https://www.pulumi.com/docs/reference/programming-model/#providers) for more information.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[derive(pulumi_gestalt_rust::__private::bon::Builder)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderArgs {
    /// Configure the base path used by the API client. Alternatively, can be configured using the `CLOUDFLARE_API_BASE_PATH`
    /// environment variable.
    #[builder(into, default)]
    pub api_base_path: pulumi_gestalt_rust::Input<Option<String>>,
    /// Whether to print logs from the API client (using the default log library logger). Alternatively, can be configured using
    /// the `CLOUDFLARE_API_CLIENT_LOGGING` environment variable.
    #[builder(into, default)]
    pub api_client_logging: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Configure the hostname used by the API client. Alternatively, can be configured using the `CLOUDFLARE_API_HOSTNAME`
    /// environment variable.
    #[builder(into, default)]
    pub api_hostname: pulumi_gestalt_rust::Input<Option<String>>,
    /// The API key for operations. Alternatively, can be configured using the `CLOUDFLARE_API_KEY` environment variable. API
    /// keys are [now considered legacy by
    /// Cloudflare](https://developers.cloudflare.com/fundamentals/api/get-started/keys/#limitations), API tokens should be used
    /// instead. Must provide only one of `api_key`, `api_token`, `api_user_service_key`.
    #[builder(into, default)]
    pub api_key: pulumi_gestalt_rust::Input<Option<String>>,
    /// The API Token for operations. Alternatively, can be configured using the `CLOUDFLARE_API_TOKEN` environment variable.
    /// Must provide only one of `api_key`, `api_token`, `api_user_service_key`.
    #[builder(into, default)]
    pub api_token: pulumi_gestalt_rust::Input<Option<String>>,
    /// A special Cloudflare API key good for a restricted set of endpoints. Alternatively, can be configured using the
    /// `CLOUDFLARE_API_USER_SERVICE_KEY` environment variable. Must provide only one of `api_key`, `api_token`,
    /// `api_user_service_key`.
    #[builder(into, default)]
    pub api_user_service_key: pulumi_gestalt_rust::Input<Option<String>>,
    /// A registered Cloudflare email address. Alternatively, can be configured using the `CLOUDFLARE_EMAIL` environment
    /// variable. Required when using `api_key`. Conflicts with `api_token`.
    #[builder(into, default)]
    pub email: pulumi_gestalt_rust::Input<Option<String>>,
    /// Maximum backoff period in seconds after failed API calls. Alternatively, can be configured using the
    /// `CLOUDFLARE_MAX_BACKOFF` environment variable.
    #[builder(into, default)]
    pub max_backoff: pulumi_gestalt_rust::Input<Option<i32>>,
    /// Minimum backoff period in seconds after failed API calls. Alternatively, can be configured using the
    /// `CLOUDFLARE_MIN_BACKOFF` environment variable.
    #[builder(into, default)]
    pub min_backoff: pulumi_gestalt_rust::Input<Option<i32>>,
    /// Maximum number of retries to perform when an API request fails. Alternatively, can be configured using the
    /// `CLOUDFLARE_RETRIES` environment variable.
    #[builder(into, default)]
    pub retries: pulumi_gestalt_rust::Input<Option<i32>>,
    /// RPS limit to apply when making calls to the API. Alternatively, can be configured using the `CLOUDFLARE_RPS` environment
    /// variable.
    #[builder(into, default)]
    pub rps: pulumi_gestalt_rust::Input<Option<i32>>,
    #[builder(into, default)]
    pub user_agent_operator_suffix: pulumi_gestalt_rust::Input<Option<String>>,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
    /// Configure the base path used by the API client. Alternatively, can be configured using the `CLOUDFLARE_API_BASE_PATH`
    /// environment variable.
    pub api_base_path: pulumi_gestalt_rust::Output<Option<String>>,
    /// Whether to print logs from the API client (using the default log library logger). Alternatively, can be configured using
    /// the `CLOUDFLARE_API_CLIENT_LOGGING` environment variable.
    pub api_client_logging: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Configure the hostname used by the API client. Alternatively, can be configured using the `CLOUDFLARE_API_HOSTNAME`
    /// environment variable.
    pub api_hostname: pulumi_gestalt_rust::Output<Option<String>>,
    /// The API key for operations. Alternatively, can be configured using the `CLOUDFLARE_API_KEY` environment variable. API
    /// keys are [now considered legacy by
    /// Cloudflare](https://developers.cloudflare.com/fundamentals/api/get-started/keys/#limitations), API tokens should be used
    /// instead. Must provide only one of `api_key`, `api_token`, `api_user_service_key`.
    pub api_key: pulumi_gestalt_rust::Output<Option<String>>,
    /// The API Token for operations. Alternatively, can be configured using the `CLOUDFLARE_API_TOKEN` environment variable.
    /// Must provide only one of `api_key`, `api_token`, `api_user_service_key`.
    pub api_token: pulumi_gestalt_rust::Output<Option<String>>,
    /// A special Cloudflare API key good for a restricted set of endpoints. Alternatively, can be configured using the
    /// `CLOUDFLARE_API_USER_SERVICE_KEY` environment variable. Must provide only one of `api_key`, `api_token`,
    /// `api_user_service_key`.
    pub api_user_service_key: pulumi_gestalt_rust::Output<Option<String>>,
    /// A registered Cloudflare email address. Alternatively, can be configured using the `CLOUDFLARE_EMAIL` environment
    /// variable. Required when using `api_key`. Conflicts with `api_token`.
    pub email: pulumi_gestalt_rust::Output<Option<String>>,
    /// Maximum backoff period in seconds after failed API calls. Alternatively, can be configured using the
    /// `CLOUDFLARE_MAX_BACKOFF` environment variable.
    pub max_backoff: pulumi_gestalt_rust::Output<Option<i32>>,
    /// Minimum backoff period in seconds after failed API calls. Alternatively, can be configured using the
    /// `CLOUDFLARE_MIN_BACKOFF` environment variable.
    pub min_backoff: pulumi_gestalt_rust::Output<Option<i32>>,
    /// Maximum number of retries to perform when an API request fails. Alternatively, can be configured using the
    /// `CLOUDFLARE_RETRIES` environment variable.
    pub retries: pulumi_gestalt_rust::Output<Option<i32>>,
    /// RPS limit to apply when making calls to the API. Alternatively, can be configured using the `CLOUDFLARE_RPS` environment
    /// variable.
    pub rps: pulumi_gestalt_rust::Output<Option<i32>>,
    pub user_agent_operator_suffix: pulumi_gestalt_rust::Output<Option<String>>,
}
impl pulumi_gestalt_rust::Provider for ProviderResult {
    fn get_provider_id(&self) -> pulumi_gestalt_rust::Output<String> {
        self.provider_id.clone()
    }
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
) -> ProviderResult {
    create_with_options(context, name, args, None)
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create_with_options(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
    options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
) -> ProviderResult {
    let api_base_path_binding = args.api_base_path.get_output(context);
    let api_client_logging_binding = args.api_client_logging.get_output(context);
    let api_hostname_binding = args.api_hostname.get_output(context);
    let api_key_binding = args.api_key.get_output(context);
    let api_token_binding = args.api_token.get_output(context);
    let api_user_service_key_binding = args.api_user_service_key.get_output(context);
    let email_binding = args.email.get_output(context);
    let max_backoff_binding = args.max_backoff.get_output(context);
    let min_backoff_binding = args.min_backoff.get_output(context);
    let retries_binding = args.retries.get_output(context);
    let rps_binding = args.rps.get_output(context);
    let user_agent_operator_suffix_binding = args
        .user_agent_operator_suffix
        .get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:cloudflare".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiBasePath".into(),
                value: &api_base_path_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiClientLogging".into(),
                value: &api_client_logging_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiHostname".into(),
                value: &api_hostname_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiKey".into(),
                value: &api_key_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiToken".into(),
                value: &api_token_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiUserServiceKey".into(),
                value: &api_user_service_key_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "email".into(),
                value: &email_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "maxBackoff".into(),
                value: &max_backoff_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "minBackoff".into(),
                value: &min_backoff_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "retries".into(),
                value: &retries_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "rps".into(),
                value: &rps_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "userAgentOperatorSuffix".into(),
                value: &user_agent_operator_suffix_binding.drop_type(),
            },
        ],
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
        api_base_path: o.get_field("apiBasePath"),
        api_client_logging: o.get_field("apiClientLogging"),
        api_hostname: o.get_field("apiHostname"),
        api_key: o.get_field("apiKey"),
        api_token: o.get_field("apiToken"),
        api_user_service_key: o.get_field("apiUserServiceKey"),
        email: o.get_field("email"),
        max_backoff: o.get_field("maxBackoff"),
        min_backoff: o.get_field("minBackoff"),
        retries: o.get_field("retries"),
        rps: o.get_field("rps"),
        user_agent_operator_suffix: o.get_field("userAgentOperatorSuffix"),
    }
}
