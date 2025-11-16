#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustAccessGroupRequireGithub {
    /// The ID of your Github identity provider.
    #[builder(into)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    /// The name of the organization.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The teams that should be matched.
    #[builder(into)]
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}
