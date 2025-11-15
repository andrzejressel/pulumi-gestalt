#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppAuthSettingsMicrosoft {
    /// The OAuth 2.0 client ID that was created for the app used for authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The OAuth 2.0 client secret that was created for the app used for authentication. Cannot be specified with `client_secret_setting_name`.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// The app setting name containing the OAuth 2.0 client secret that was created for the app used for authentication. Cannot be specified with `client_secret`.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Option<String>,
    /// Specifies a list of OAuth 2.0 scopes that will be requested as part of Microsoft Account authentication. If not specified, "wl.basic" is used as the default scope.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Option<Vec<String>>,
}
