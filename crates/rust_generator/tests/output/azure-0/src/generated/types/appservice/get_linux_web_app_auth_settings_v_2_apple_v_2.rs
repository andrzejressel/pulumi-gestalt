#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxWebAppAuthSettingsV2AppleV2 {
    /// The OAuth 2.0 client ID used by the app for authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The app setting name containing the OAuth 2.0 client secret used by the app for authentication.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<String>,
    /// The list of Login scopes that are requested as part of Microsoft Account authentication.
    #[builder(into)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Box<Vec<String>>,
}
