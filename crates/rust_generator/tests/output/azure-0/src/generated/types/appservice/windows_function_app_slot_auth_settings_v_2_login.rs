#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSlotAuthSettingsV2Login {
    /// External URLs that can be redirected to as part of logging in or logging out of the app. This is an advanced setting typically only needed by Windows Store application backends.
    /// 
    /// > **Note:** URLs within the current domain are always implicitly allowed.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Option<Vec<String>>,
    /// The method by which cookies expire. Possible values include: `FixedTime`, and `IdentityProviderDerived`. Defaults to `FixedTime`.
    #[builder(into)]
    #[serde(rename = "cookieExpirationConvention")]
    pub r#cookie_expiration_convention: Option<String>,
    /// The time after the request is made when the session cookie should expire. Defaults to `08:00:00`.
    #[builder(into)]
    #[serde(rename = "cookieExpirationTime")]
    pub r#cookie_expiration_time: Option<String>,
    /// The endpoint to which logout requests should be made.
    #[builder(into)]
    #[serde(rename = "logoutEndpoint")]
    pub r#logout_endpoint: Option<String>,
    /// The time after the request is made when the nonce should expire. Defaults to `00:05:00`.
    #[builder(into)]
    #[serde(rename = "nonceExpirationTime")]
    pub r#nonce_expiration_time: Option<String>,
    /// Should the fragments from the request be preserved after the login request is made. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "preserveUrlFragmentsForLogins")]
    pub r#preserve_url_fragments_for_logins: Option<bool>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72` hours.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionTime")]
    pub r#token_refresh_extension_time: Option<f64>,
    /// Should the Token Store configuration Enabled. Defaults to `false`
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Option<bool>,
    /// The directory path in the App Filesystem in which the tokens will be stored.
    #[builder(into)]
    #[serde(rename = "tokenStorePath")]
    pub r#token_store_path: Option<String>,
    /// The name of the app setting which contains the SAS URL of the blob storage containing the tokens.
    #[builder(into)]
    #[serde(rename = "tokenStoreSasSettingName")]
    pub r#token_store_sas_setting_name: Option<String>,
    /// Should the nonce be validated while completing the login flow. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "validateNonce")]
    pub r#validate_nonce: Option<bool>,
}
