#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerRepositoryEventConfig {
    /// Contains filter properties for matching Pull Requests.
    #[builder(into)]
    #[serde(rename = "pullRequests")]
    pub r#pull_requests: Vec<super::super::types::cloudbuild::GetTriggerRepositoryEventConfigPullRequest>,
    /// Contains filter properties for matching git pushes.
    #[builder(into)]
    #[serde(rename = "pushes")]
    pub r#pushes: Vec<super::super::types::cloudbuild::GetTriggerRepositoryEventConfigPush>,
    /// The resource name of the Repo API resource.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: String,
}
