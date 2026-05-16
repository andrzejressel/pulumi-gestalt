#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRepositoryRemoteRepositoryConfigMavenRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    #[builder(into)]
    #[serde(rename = "customRepositories")]
    pub r#custom_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigMavenRepositoryCustomRepository>,
    /// Address of the remote repository. Default value: "MAVEN_CENTRAL" Possible values: ["MAVEN_CENTRAL"]
    #[builder(into)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: String,
}
