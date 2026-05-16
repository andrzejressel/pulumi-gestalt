#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryRemoteRepositoryConfigNpmRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customRepository")]
    pub r#custom_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigNpmRepositoryCustomRepository>>,
    /// Address of the remote repository.
    /// Default value is `NPMJS`.
    /// Possible values are: `NPMJS`.
    #[builder(into)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Option<String>,
}
