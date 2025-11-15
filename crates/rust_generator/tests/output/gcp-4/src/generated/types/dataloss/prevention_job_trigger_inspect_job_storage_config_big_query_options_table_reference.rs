#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsTableReference {
    /// The dataset ID of the table.
    #[builder(into)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: String,
    /// The Google Cloud Platform project ID of the project containing the table.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
    /// The name of the table.
    #[builder(into)]
    #[serde(rename = "tableId")]
    pub r#table_id: String,
}
