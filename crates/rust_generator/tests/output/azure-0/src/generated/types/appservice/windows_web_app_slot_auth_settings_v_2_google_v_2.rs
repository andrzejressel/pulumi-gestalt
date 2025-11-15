#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotAuthSettingsV2GoogleV2 {
    /// Specifies a list of Allowed Audiences that should be requested as part of Google Sign-In authentication.
    #[builder(into)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Option<Vec<String>>,
    /// The OpenID Connect Client ID for the Google web application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The app setting name that contains the `client_secret` value used for Google Login.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: String,
    /// The list of OAuth 2.0 scopes that should be requested as part of Google Sign-In authentication.
    #[builder(into)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Option<Vec<String>>,
}
