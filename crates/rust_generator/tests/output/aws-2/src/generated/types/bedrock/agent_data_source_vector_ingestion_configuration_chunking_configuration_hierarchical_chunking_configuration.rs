#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration {
    /// Maximum number of tokens to include in a chunk. Must contain two `level_configurations`. See `level_configurations` for details.
    #[builder(into)]
    #[serde(rename = "levelConfigurations")]
    pub r#level_configurations: Vec<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfigurationLevelConfiguration>,
    /// The number of tokens to repeat across chunks in the same layer.
    #[builder(into)]
    #[serde(rename = "overlapTokens")]
    pub r#overlap_tokens: f64,
}
