#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BitbucketServerConfigConnectedRepository {
    /// Identifier for the project storing the repository.
    #[builder(into)]
    #[serde(rename = "projectKey")]
    pub r#project_key: String,
    /// Identifier for the repository.
    #[builder(into)]
    #[serde(rename = "repoSlug")]
    pub r#repo_slug: String,
}
