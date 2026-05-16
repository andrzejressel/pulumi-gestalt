#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppServiceAuthSettingsTwitter {
    /// The consumer key of the Twitter app used for login
    #[builder(into)]
    #[serde(rename = "consumerKey")]
    pub r#consumer_key: String,
    /// The consumer secret of the Twitter app used for login.
    #[builder(into)]
    #[serde(rename = "consumerSecret")]
    pub r#consumer_secret: String,
}
