#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetLogicalTableMapSource {
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: String,
    #[builder(into)]
    #[serde(rename = "joinInstructions")]
    pub r#join_instructions: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstruction>,
    #[builder(into)]
    #[serde(rename = "physicalTableId")]
    pub r#physical_table_id: String,
}
