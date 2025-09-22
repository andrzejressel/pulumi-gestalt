#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetLogicalTableMapDataTransformUntagColumnOperation {
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: String,
    #[builder(into)]
    #[serde(rename = "tagNames")]
    pub r#tag_names: Vec<String>,
}
