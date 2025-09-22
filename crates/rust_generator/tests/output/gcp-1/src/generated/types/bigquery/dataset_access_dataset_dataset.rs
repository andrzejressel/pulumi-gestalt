#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatasetAccessDatasetDataset {
    /// The ID of the dataset containing this table.
    #[builder(into)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: String,
    /// The ID of the project containing this table.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
}
