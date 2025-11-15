#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureNextStepIntent {
    /// Name of the intent.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Configuration block for all of the slot value overrides for the intent. The name of the slot maps to the value of the slot. Slots that are not included in the map aren't overridden. See `slot`.
    #[builder(into)]
    #[serde(rename = "slots")]
    pub r#slots: Option<Vec<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureNextStepIntentSlot>>,
}
