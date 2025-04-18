#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryRemoteRepositoryConfigAptRepository {
    /// One of the publicly available Apt repositories supported by Artifact Registry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<Option<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigAptRepositoryPublicRepository>>,
}
