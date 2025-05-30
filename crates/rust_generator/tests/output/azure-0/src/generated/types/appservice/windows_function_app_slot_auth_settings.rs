#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSlotAuthSettings {
    /// an `active_directory` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsActiveDirectory>>,
    /// Specifies a map of login Parameters to send to the OpenID Connect authorization endpoint when a user logs in.
    #[builder(into, default)]
    #[serde(rename = "additionalLoginParameters")]
    pub r#additional_login_parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies a list of External URLs that can be redirected to as part of logging in or logging out of the Windows Web App.
    #[builder(into, default)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Box<Option<Vec<String>>>,
    /// The default authentication provider to use when multiple providers are configured. Possible values include: `AzureActiveDirectory`, `Facebook`, `Google`, `MicrosoftAccount`, `Twitter`, `Github`.
    /// 
    /// > **NOTE:** This setting is only needed if multiple providers are configured, and the `unauthenticated_client_action` is set to "RedirectToLoginPage".
    #[builder(into, default)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: Box<Option<String>>,
    /// Should the Authentication / Authorization feature be enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// a `facebook` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "facebook")]
    pub r#facebook: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsFacebook>>,
    /// a `github` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "github")]
    pub r#github: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsGithub>>,
    /// a `google` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "google")]
    pub r#google: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsGoogle>>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens.
    /// 
    /// > **NOTE:** When using Azure Active Directory, this value is the URI of the directory tenant, e.g. <https://sts.windows.net/{tenant-guid}/>.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    /// a `microsoft` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "microsoft")]
    pub r#microsoft: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsMicrosoft>>,
    /// The RuntimeVersion of the Authentication / Authorization feature in use.
    #[builder(into, default)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<Option<String>>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72` hours.
    #[builder(into, default)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: Box<Option<f64>>,
    /// Should the Windows Web App durably store platform-specific security tokens that are obtained during login flows? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Box<Option<bool>>,
    /// a `twitter` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "twitter")]
    pub r#twitter: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsTwitter>>,
    /// The action to take when an unauthenticated client attempts to access the app. Possible values include: `RedirectToLoginPage`, `AllowAnonymous`.
    #[builder(into, default)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: Box<Option<String>>,
}
