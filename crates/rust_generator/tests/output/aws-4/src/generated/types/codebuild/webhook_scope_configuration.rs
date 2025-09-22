#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebhookScopeConfiguration {
    /// The domain of the GitHub Enterprise organization. Required if your project's source type is GITHUB_ENTERPRISE.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// The name of either the enterprise or organization.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of scope for a GitHub webhook. Valid values for this parameter are: `GITHUB_ORGANIZATION`, `GITHUB_GLOBAL`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
}
