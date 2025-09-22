#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDistributionConfigurationDistributionContainerDistributionConfiguration {
    /// Set of tags that are attached to the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Vec<String>,
    /// Description of the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Set of destination repositories for the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "targetRepositories")]
    pub r#target_repositories: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository>,
}
