#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryInitialConfig {
    /// Default branch name of the repository.
    #[builder(into)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: Option<String>,
    /// List of gitignore template names user can choose from.
    /// Valid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig.
    #[builder(into)]
    #[serde(rename = "gitignores")]
    pub r#gitignores: Option<Vec<String>>,
    /// License template name user can choose from.
    /// Valid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig.
    #[builder(into)]
    #[serde(rename = "license")]
    pub r#license: Option<String>,
    /// README template name.
    /// Valid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig.
    #[builder(into)]
    #[serde(rename = "readme")]
    pub r#readme: Option<String>,
}
