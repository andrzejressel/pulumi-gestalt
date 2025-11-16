#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceAzureDevopsRepo {
    /// Specifies the Azure DevOps account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// Specifies the collaboration branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// The last commit ID.
    #[builder(into)]
    #[serde(rename = "lastCommitId")]
    pub r#last_commit_id: Option<String>,
    /// Specifies the name of the Azure DevOps project.
    #[builder(into)]
    #[serde(rename = "projectName")]
    pub r#project_name: String,
    /// Specifies the name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: String,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: String,
    /// the ID of the tenant for the Azure DevOps account.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}
