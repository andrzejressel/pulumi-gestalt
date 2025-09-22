#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
