#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ImagePipelineImageScanningConfigurationEcrConfiguration {
    #[builder(into)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Option<Vec<String>>,
    /// The name of the repository to scan
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Option<String>,
}
