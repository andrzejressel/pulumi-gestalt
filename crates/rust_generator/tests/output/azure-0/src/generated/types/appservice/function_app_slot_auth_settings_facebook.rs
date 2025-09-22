#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSlotAuthSettingsFacebook {
    /// The App ID of the Facebook app used for login
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: String,
    /// The App Secret of the Facebook app used for Facebook login.
    #[builder(into)]
    #[serde(rename = "appSecret")]
    pub r#app_secret: String,
    /// The OAuth 2.0 scopes that will be requested as part of Facebook login authentication. <https://developers.facebook.com/docs/facebook-login>
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Option<Vec<String>>,
}
