#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetLogicalTableMapSourceJoinInstruction {
    #[builder(into)]
    #[serde(rename = "leftJoinKeyProperties")]
    pub r#left_join_key_properties: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty>,
    #[builder(into)]
    #[serde(rename = "leftOperand")]
    pub r#left_operand: String,
    #[builder(into)]
    #[serde(rename = "onClause")]
    pub r#on_clause: String,
    #[builder(into)]
    #[serde(rename = "rightJoinKeyProperties")]
    pub r#right_join_key_properties: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty>,
    #[builder(into)]
    #[serde(rename = "rightOperand")]
    pub r#right_operand: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
