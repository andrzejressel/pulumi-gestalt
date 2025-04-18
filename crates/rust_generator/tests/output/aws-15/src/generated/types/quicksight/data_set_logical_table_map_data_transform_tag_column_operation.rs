#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetLogicalTableMapDataTransformTagColumnOperation {
    /// Column name.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// The dataset column tag, currently only used for geospatial type tagging. See tags.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<super::super::types::quicksight::DataSetLogicalTableMapDataTransformTagColumnOperationTag>>,
}
