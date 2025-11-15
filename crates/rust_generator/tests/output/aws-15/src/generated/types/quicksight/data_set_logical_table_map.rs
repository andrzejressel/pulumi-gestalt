#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetLogicalTableMap {
    /// A display name for the logical table.
    #[builder(into)]
    #[serde(rename = "alias")]
    pub r#alias: String,
    /// Transform operations that act on this logical table. For this structure to be valid, only one of the attributes can be non-null. See data_transforms.
    #[builder(into)]
    #[serde(rename = "dataTransforms")]
    pub r#data_transforms: Option<Vec<super::super::types::quicksight::DataSetLogicalTableMapDataTransform>>,
    /// Key of the logical table map.
    #[builder(into)]
    #[serde(rename = "logicalTableMapId")]
    pub r#logical_table_map_id: String,
    /// Source of this logical table. See source.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::super::types::quicksight::DataSetLogicalTableMapSource>,
}
