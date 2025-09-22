#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxFunctionAppAuthSettingTwitter {
    /// The OAuth 1.0a consumer key of the Twitter application used for sign-in.
    #[builder(into)]
    #[serde(rename = "consumerKey")]
    pub r#consumer_key: String,
    /// The OAuth 1.0a consumer secret of the Twitter application used for sign-in.
    #[builder(into)]
    #[serde(rename = "consumerSecret")]
    pub r#consumer_secret: String,
    /// The app setting name that contains the OAuth 1.0a consumer secret of the Twitter application used for sign-in.
    #[builder(into)]
    #[serde(rename = "consumerSecretSettingName")]
    pub r#consumer_secret_setting_name: String,
}
