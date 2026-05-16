#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogTablePartitionIndex {
    /// Name of the partition index.
    #[builder(into)]
    #[serde(rename = "indexName")]
    pub r#index_name: String,
    #[builder(into)]
    #[serde(rename = "indexStatus")]
    pub r#index_status: Option<String>,
    /// Keys for the partition index.
    #[builder(into)]
    #[serde(rename = "keys")]
    pub r#keys: Vec<String>,
}
