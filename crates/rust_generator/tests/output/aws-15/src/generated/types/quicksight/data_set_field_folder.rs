#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetFieldFolder {
    /// An array of column names to add to the folder. A column can only be in one folder.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Option<Vec<String>>,
    /// Field folder description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Key of the field folder map.
    #[builder(into)]
    #[serde(rename = "fieldFoldersId")]
    pub r#field_folders_id: String,
}
