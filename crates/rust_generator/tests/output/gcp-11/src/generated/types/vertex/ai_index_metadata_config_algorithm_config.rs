#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiIndexMetadataConfigAlgorithmConfig {
    /// Configuration options for using brute force search, which simply implements the
    /// standard linear search in the database for each query.
    #[builder(into)]
    #[serde(rename = "bruteForceConfig")]
    pub r#brute_force_config: Option<Box<super::super::types::vertex::AiIndexMetadataConfigAlgorithmConfigBruteForceConfig>>,
    /// Configuration options for using the tree-AH algorithm (Shallow tree + Asymmetric Hashing).
    /// Please refer to this paper for more details: https://arxiv.org/abs/1908.10396
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "treeAhConfig")]
    pub r#tree_ah_config: Option<Box<super::super::types::vertex::AiIndexMetadataConfigAlgorithmConfigTreeAhConfig>>,
}
