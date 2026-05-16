#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
