#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerGithub {
    /// The resource name of the github enterprise config that should be applied to this installation.
    /// For example: "projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}"
    #[builder(into)]
    #[serde(rename = "enterpriseConfigResourceName")]
    pub r#enterprise_config_resource_name: Option<String>,
    /// Name of the repository. For example: The name for
    /// https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Owner of the repository. For example: The owner for
    /// https://github.com/googlecloudplatform/cloud-builders is "googlecloudplatform".
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Option<String>,
    /// filter to match changes in pull requests. Specify only one of `pull_request` or `push`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pullRequest")]
    pub r#pull_request: Option<Box<super::super::types::cloudbuild::TriggerGithubPullRequest>>,
    /// filter to match changes in refs, like branches or tags. Specify only one of `pull_request` or `push`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "push")]
    pub r#push: Option<Box<super::super::types::cloudbuild::TriggerGithubPush>>,
}
