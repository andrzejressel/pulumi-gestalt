#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppAuthSettings {
    /// An `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Option<Box<super::super::types::appservice::WindowsWebAppAuthSettingsActiveDirectory>>,
    /// Specifies a map of login Parameters to send to the OpenID Connect authorization endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "additionalLoginParameters")]
    pub r#additional_login_parameters: Option<std::collections::HashMap<String, String>>,
    /// Specifies a list of External URLs that can be redirected to as part of logging in or logging out of the Windows Web App.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Option<Vec<String>>,
    /// The default authentication provider to use when multiple providers are configured. Possible values include: `AzureActiveDirectory`, `Facebook`, `Google`, `MicrosoftAccount`, `Twitter`, `Github`
    /// 
    /// > **NOTE:** This setting is only needed if multiple providers are configured, and the `unauthenticated_client_action` is set to "RedirectToLoginPage".
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: Option<String>,
    /// Should the Authentication / Authorization feature is enabled for the Windows Web App be enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A `facebook` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebook")]
    pub r#facebook: Option<Box<super::super::types::appservice::WindowsWebAppAuthSettingsFacebook>>,
    /// A `github` block as defined below.
    #[builder(into)]
    #[serde(rename = "github")]
    pub r#github: Option<Box<super::super::types::appservice::WindowsWebAppAuthSettingsGithub>>,
    /// A `google` block as defined below.
    #[builder(into)]
    #[serde(rename = "google")]
    pub r#google: Option<Box<super::super::types::appservice::WindowsWebAppAuthSettingsGoogle>>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens for this Windows Web App.
    /// 
    /// > **NOTE:** When using Azure Active Directory, this value is the URI of the directory tenant, e.g. <https://sts.windows.net/{tenant-guid}/>.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// A `microsoft` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsoft")]
    pub r#microsoft: Option<Box<super::super::types::appservice::WindowsWebAppAuthSettingsMicrosoft>>,
    /// The RuntimeVersion of the Authentication / Authorization feature in use for the Windows Web App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Option<String>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72` hours.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: Option<f64>,
    /// Should the Windows Web App durably store platform-specific security tokens that are obtained during login flows? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Option<bool>,
    /// A `twitter` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitter")]
    pub r#twitter: Option<Box<super::super::types::appservice::WindowsWebAppAuthSettingsTwitter>>,
    /// The action to take when an unauthenticated client attempts to access the app. Possible values include: `RedirectToLoginPage`, `AllowAnonymous`.
    #[builder(into)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: Option<String>,
}
