#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiIndexIndexStat {
    /// (Output)
    /// The number of shards in the Index.
    #[builder(into)]
    #[serde(rename = "shardsCount")]
    pub r#shards_count: Option<i32>,
    /// (Output)
    /// The number of vectors in the Index.
    #[builder(into)]
    #[serde(rename = "vectorsCount")]
    pub r#vectors_count: Option<String>,
}
