#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAiIndexMetadataConfigAlgorithmConfig {
    /// Configuration options for using brute force search, which simply implements the
    /// standard linear search in the database for each query.
    #[builder(into)]
    #[serde(rename = "bruteForceConfigs")]
    pub r#brute_force_configs: Vec<super::super::types::vertex::GetAiIndexMetadataConfigAlgorithmConfigBruteForceConfig>,
    /// Configuration options for using the tree-AH algorithm (Shallow tree + Asymmetric Hashing).
    /// Please refer to this paper for more details: https://arxiv.org/abs/1908.10396
    #[builder(into)]
    #[serde(rename = "treeAhConfigs")]
    pub r#tree_ah_configs: Vec<super::super::types::vertex::GetAiIndexMetadataConfigAlgorithmConfigTreeAhConfig>,
}
