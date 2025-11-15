#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxAgentGitIntegrationSettingsGithubSettings {
    /// The access token used to authenticate the access to the GitHub repository.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Option<String>,
    /// A list of branches configured to be used from Dialogflow.
    #[builder(into)]
    #[serde(rename = "branches")]
    pub r#branches: Option<Vec<String>>,
    /// The unique repository display name for the GitHub repository.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The GitHub repository URI related to the agent.
    #[builder(into)]
    #[serde(rename = "repositoryUri")]
    pub r#repository_uri: Option<String>,
    /// The branch of the GitHub repository tracked for this agent.
    #[builder(into)]
    #[serde(rename = "trackingBranch")]
    pub r#tracking_branch: Option<String>,
}
