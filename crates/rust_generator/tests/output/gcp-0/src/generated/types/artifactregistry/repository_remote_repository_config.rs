#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryRemoteRepositoryConfig {
    /// Specific settings for an Apt remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "aptRepository")]
    pub r#apt_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigAptRepository>>,
    /// Specific settings for an Artifact Registory remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "commonRepository")]
    pub r#common_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigCommonRepository>>,
    /// The description of the remote source.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// If true, the remote repository upstream and upstream credentials will
    /// not be validated.
    #[builder(into)]
    #[serde(rename = "disableUpstreamValidation")]
    pub r#disable_upstream_validation: Option<bool>,
    /// Specific settings for a Docker remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dockerRepository")]
    pub r#docker_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigDockerRepository>>,
    /// Specific settings for a Maven remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mavenRepository")]
    pub r#maven_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigMavenRepository>>,
    /// Specific settings for an Npm remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "npmRepository")]
    pub r#npm_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigNpmRepository>>,
    /// Specific settings for a Python remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pythonRepository")]
    pub r#python_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigPythonRepository>>,
    /// The credentials used to access the remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "upstreamCredentials")]
    pub r#upstream_credentials: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigUpstreamCredentials>>,
    /// Specific settings for an Yum remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "yumRepository")]
    pub r#yum_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigYumRepository>>,
}
