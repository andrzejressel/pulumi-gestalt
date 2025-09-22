#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationFixedSizeChunkingConfiguration {
    /// Maximum number of tokens to include in a chunk.
    #[builder(into)]
    #[serde(rename = "maxTokens")]
    pub r#max_tokens: i32,
    /// Percentage of overlap between adjacent chunks of a data source.
    #[builder(into)]
    #[serde(rename = "overlapPercentage")]
    pub r#overlap_percentage: i32,
}
