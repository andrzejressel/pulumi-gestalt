#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryAssociationRepository {
    #[builder(into, default)]
    #[serde(rename = "bitbucket")]
    pub r#bitbucket: Box<Option<super::super::types::codegurureviewer::RepositoryAssociationRepositoryBitbucket>>,
    #[builder(into, default)]
    #[serde(rename = "codecommit")]
    pub r#codecommit: Box<Option<super::super::types::codegurureviewer::RepositoryAssociationRepositoryCodecommit>>,
    #[builder(into, default)]
    #[serde(rename = "githubEnterpriseServer")]
    pub r#github_enterprise_server: Box<Option<super::super::types::codegurureviewer::RepositoryAssociationRepositoryGithubEnterpriseServer>>,
    #[builder(into, default)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<Option<super::super::types::codegurureviewer::RepositoryAssociationRepositoryS3Bucket>>,
}
