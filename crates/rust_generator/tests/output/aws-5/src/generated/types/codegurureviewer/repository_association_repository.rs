#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryAssociationRepository {
    #[builder(into)]
    #[serde(rename = "bitbucket")]
    pub r#bitbucket: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryBitbucket>>,
    #[builder(into)]
    #[serde(rename = "codecommit")]
    pub r#codecommit: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryCodecommit>>,
    #[builder(into)]
    #[serde(rename = "githubEnterpriseServer")]
    pub r#github_enterprise_server: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryGithubEnterpriseServer>>,
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryS3Bucket>>,
}
