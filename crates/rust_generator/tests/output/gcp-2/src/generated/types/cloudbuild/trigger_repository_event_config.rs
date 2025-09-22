#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerRepositoryEventConfig {
    /// Contains filter properties for matching Pull Requests.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pullRequest")]
    pub r#pull_request: Box<Option<super::super::types::cloudbuild::TriggerRepositoryEventConfigPullRequest>>,
    /// Contains filter properties for matching git pushes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "push")]
    pub r#push: Box<Option<super::super::types::cloudbuild::TriggerRepositoryEventConfigPush>>,
    /// The resource name of the Repo API resource.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Option<String>,
}
