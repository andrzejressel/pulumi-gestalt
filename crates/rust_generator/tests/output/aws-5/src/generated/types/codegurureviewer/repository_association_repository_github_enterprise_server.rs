#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryAssociationRepositoryGithubEnterpriseServer {
    /// The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
    #[builder(into)]
    #[serde(rename = "connectionArn")]
    pub r#connection_arn: Box<String>,
    /// The name of the third party source repository.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The username for the account that owns the repository.
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Box<String>,
}
