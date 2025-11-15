#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineTriggerGitConfigurationPullRequest {
    /// The field that specifies to filter on branches for the pull request trigger configuration. A `branches` block is documented below.
    #[builder(into)]
    #[serde(rename = "branches")]
    pub r#branches: Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPullRequestBranches>>,
    /// A list that specifies which pull request events to filter on (opened, updated, closed) for the trigger configuration. Possible values are `OPEN`, `UPDATED ` and `CLOSED`.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Option<Vec<String>>,
    /// The field that specifies to filter on file paths for the pull request trigger configuration. A `file_paths` block is documented below.
    #[builder(into)]
    #[serde(rename = "filePaths")]
    pub r#file_paths: Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPullRequestFilePaths>>,
}
