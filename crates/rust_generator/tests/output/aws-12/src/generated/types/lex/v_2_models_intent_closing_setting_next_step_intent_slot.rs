#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentClosingSettingNextStepIntentSlot {
    /// Which attempt to configure. Valid values are `Initial`, `Retry1`, `Retry2`, `Retry3`, `Retry4`, `Retry5`.
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: String,
    /// When the shape value is `List`, `values` contains a list of slot values. When the value is `Scalar`, `value` contains a single value.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: Option<String>,
    /// Configuration block for the current value of the slot. See `value`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingNextStepIntentSlotValue>>,
}
