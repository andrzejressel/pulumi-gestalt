#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxFunctionAppAuthSettingsV2Login {
    /// External URLs that can be redirected to as part of logging in or logging out of the app.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Vec<String>,
    /// The method by which cookies expire.
    #[builder(into)]
    #[serde(rename = "cookieExpirationConvention")]
    pub r#cookie_expiration_convention: String,
    /// The time after the request is made when the session cookie should expire.
    #[builder(into)]
    #[serde(rename = "cookieExpirationTime")]
    pub r#cookie_expiration_time: String,
    /// The endpoint to which logout requests are made.
    #[builder(into)]
    #[serde(rename = "logoutEndpoint")]
    pub r#logout_endpoint: String,
    /// The time after the request is made when the nonce should expire.
    #[builder(into)]
    #[serde(rename = "nonceExpirationTime")]
    pub r#nonce_expiration_time: String,
    /// Are the fragments from the request preserved after the login request is made.
    #[builder(into)]
    #[serde(rename = "preserveUrlFragmentsForLogins")]
    pub r#preserve_url_fragments_for_logins: bool,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionTime")]
    pub r#token_refresh_extension_time: f64,
    /// Is the Token Store configuration Enabled.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: bool,
    /// The directory path in the App Filesystem in which the tokens are stored.
    #[builder(into)]
    #[serde(rename = "tokenStorePath")]
    pub r#token_store_path: String,
    /// The name of the app setting which contains the SAS URL of the blob storage containing the tokens.
    #[builder(into)]
    #[serde(rename = "tokenStoreSasSettingName")]
    pub r#token_store_sas_setting_name: String,
    /// Is the nonce validated while completing the login flow.
    #[builder(into)]
    #[serde(rename = "validateNonce")]
    pub r#validate_nonce: bool,
}
