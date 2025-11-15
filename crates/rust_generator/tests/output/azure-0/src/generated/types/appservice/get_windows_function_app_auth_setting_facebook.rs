#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsFunctionAppAuthSettingFacebook {
    /// The App ID of the Facebook app used for login.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: String,
    /// The App Secret of the Facebook app used for Facebook Login.
    #[builder(into)]
    #[serde(rename = "appSecret")]
    pub r#app_secret: String,
    /// The app setting name that contains the `app_secret` value used for Facebook Login.
    #[builder(into)]
    #[serde(rename = "appSecretSettingName")]
    pub r#app_secret_setting_name: String,
    /// A list of OAuth 2.0 scopes that will be requested as part of Microsoft Account authentication.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Vec<String>,
}
