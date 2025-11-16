#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbServersDbServerProperty {
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeIds")]
    pub r#db_node_ids: Vec<String>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageSizeGb")]
    pub r#max_db_node_storage_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxMemorySizeGb")]
    pub r#max_memory_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxOcpuCount")]
    pub r#max_ocpu_count: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// Output only
    #[builder(into)]
    #[serde(rename = "ocpuCount")]
    pub r#ocpu_count: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// Output only
    #[builder(into)]
    #[serde(rename = "vmCount")]
    pub r#vm_count: i32,
}
