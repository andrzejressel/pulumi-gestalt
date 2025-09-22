#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotSubSlotSetting {
    /// Expression text for defining the constituent sub slots in the composite slot using logical `AND` and `OR` operators.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    /// Specifications for the constituent sub slots of a composite slot.
    /// See the `slot_specification` argument reference below.
    #[builder(into)]
    #[serde(rename = "slotSpecifications")]
    pub r#slot_specifications: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecification>>,
}
