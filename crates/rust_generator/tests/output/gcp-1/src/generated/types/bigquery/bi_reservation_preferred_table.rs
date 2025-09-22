#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BiReservationPreferredTable {
    /// The ID of the dataset in the above project.
    #[builder(into)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Option<String>,
    /// The assigned project ID of the project.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
    /// The ID of the table in the above dataset.
    #[builder(into)]
    #[serde(rename = "tableId")]
    pub r#table_id: Option<String>,
}
