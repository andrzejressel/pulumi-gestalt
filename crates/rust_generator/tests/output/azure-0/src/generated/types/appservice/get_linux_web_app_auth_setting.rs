#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxWebAppAuthSetting {
    /// A `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectories")]
    pub r#active_directories: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingActiveDirectory>,
    /// A `additional_login_parameters` block as defined above.
    #[builder(into)]
    #[serde(rename = "additionalLoginParameters")]
    pub r#additional_login_parameters: std::collections::HashMap<String, String>,
    /// External URLs that can be redirected to as part of logging in or logging out of the app.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Vec<String>,
    /// The Default Authentication Provider used when more than one Authentication Provider is configured and the `unauthenticated_action` is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: String,
    /// Is the Backup enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A `facebook` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebooks")]
    pub r#facebooks: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingFacebook>,
    /// A `github` block as defined below.
    #[builder(into)]
    #[serde(rename = "githubs")]
    pub r#githubs: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingGithub>,
    /// A `google` block as defined below.
    #[builder(into)]
    #[serde(rename = "googles")]
    pub r#googles: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingGoogle>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens for this Linux Web App.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// A `microsoft` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsofts")]
    pub r#microsofts: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingMicrosoft>,
    /// The Runtime Version of the Authentication and Authorisation feature of this App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: String,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: f64,
    /// Is the Token Store configuration Enabled.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: bool,
    /// A `twitter` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitters")]
    pub r#twitters: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingTwitter>,
    /// The action to take when an unauthenticated client attempts to access the app.
    #[builder(into)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: String,
}
