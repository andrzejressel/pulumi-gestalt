#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppAuthorizationConnectionAuthRequest {
    /// The authorization code returned by the application after permission is granted in the application OAuth page (after clicking on the AuthURL)..
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: String,
    /// The redirect URL that is specified in the AuthURL and the application client.
    #[builder(into)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: String,
}
