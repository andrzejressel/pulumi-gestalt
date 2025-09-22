#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CodeRepositoryGitConfig {
    /// The default branch for the Git repository.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// The URL where the Git repository is located.
    #[builder(into)]
    #[serde(rename = "repositoryUrl")]
    pub r#repository_url: String,
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the credentials used to access the git repository. The secret must have a staging label of AWSCURRENT and must be in the following format: `{"username": UserName, "password": Password}`
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
}
