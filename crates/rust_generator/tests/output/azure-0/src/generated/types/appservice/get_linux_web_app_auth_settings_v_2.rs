#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxWebAppAuthSettingsV2 {
    /// An `active_directory_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "activeDirectoryV2s")]
    pub r#active_directory_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2ActiveDirectoryV2>,
    /// An `apple_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "appleV2s")]
    pub r#apple_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2AppleV2>,
    /// Are the AuthV2 Settings enabled.
    #[builder(into)]
    #[serde(rename = "authEnabled")]
    pub r#auth_enabled: bool,
    /// An `azure_static_web_app_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "azureStaticWebAppV2s")]
    pub r#azure_static_web_app_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2AzureStaticWebAppV2>,
    /// The path to the App Auth settings.
    #[builder(into)]
    #[serde(rename = "configFilePath")]
    pub r#config_file_path: String,
    /// Zero or more `custom_oidc_v2` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customOidcV2s")]
    pub r#custom_oidc_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2CustomOidcV2>,
    /// The Default Authentication Provider used when more than one Authentication Provider is configured and the `unauthenticated_action` is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: String,
    /// The paths which should be excluded from the `unauthenticated_action` when it is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Vec<String>,
    /// A `facebook_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebookV2s")]
    pub r#facebook_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2FacebookV2>,
    /// The convention used to determine the url of the request made.
    #[builder(into)]
    #[serde(rename = "forwardProxyConvention")]
    pub r#forward_proxy_convention: String,
    /// The name of the custom header containing the host of the request.
    #[builder(into)]
    #[serde(rename = "forwardProxyCustomHostHeaderName")]
    pub r#forward_proxy_custom_host_header_name: String,
    /// The name of the custom header containing the scheme of the request.
    #[builder(into)]
    #[serde(rename = "forwardProxyCustomSchemeHeaderName")]
    pub r#forward_proxy_custom_scheme_header_name: String,
    /// A `github_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "githubV2s")]
    pub r#github_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2GithubV2>,
    /// A `google_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "googleV2s")]
    pub r#google_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2GoogleV2>,
    /// The prefix that should precede all the authentication and authorisation paths.
    #[builder(into)]
    #[serde(rename = "httpRouteApiPrefix")]
    pub r#http_route_api_prefix: String,
    /// A `login` block as defined below.
    #[builder(into)]
    #[serde(rename = "logins")]
    pub r#logins: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2Login>,
    /// A `microsoft_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsoftV2s")]
    pub r#microsoft_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2MicrosoftV2>,
    /// Is the authentication flow used for all requests.
    #[builder(into)]
    #[serde(rename = "requireAuthentication")]
    pub r#require_authentication: bool,
    /// Is HTTPS required on connections?
    #[builder(into)]
    #[serde(rename = "requireHttps")]
    pub r#require_https: bool,
    /// The Runtime Version of the Authentication and Authorisation feature of this App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: String,
    /// A `twitter_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitterV2s")]
    pub r#twitter_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2TwitterV2>,
    /// The action to take for requests made without authentication.
    #[builder(into)]
    #[serde(rename = "unauthenticatedAction")]
    pub r#unauthenticated_action: String,
}
