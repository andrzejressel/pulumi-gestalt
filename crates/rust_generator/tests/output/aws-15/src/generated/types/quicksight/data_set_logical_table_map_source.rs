#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetLogicalTableMapSource {
    /// ARN of the parent data set.
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: Option<String>,
    /// Specifies the result of a join of two logical tables. See join_instruction.
    #[builder(into)]
    #[serde(rename = "joinInstruction")]
    pub r#join_instruction: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapSourceJoinInstruction>>,
    /// Physical table ID.
    #[builder(into)]
    #[serde(rename = "physicalTableId")]
    pub r#physical_table_id: Option<String>,
}
