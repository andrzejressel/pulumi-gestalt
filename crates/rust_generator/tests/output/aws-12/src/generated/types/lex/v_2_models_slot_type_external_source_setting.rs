#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotTypeExternalSourceSetting {
    /// Settings required for a slot type based on a grammar that you provide.
    /// See `grammar_slot_type_setting` argument reference below.
    #[builder(into)]
    #[serde(rename = "grammarSlotTypeSetting")]
    pub r#grammar_slot_type_setting: Option<Box<super::super::types::lex::V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSetting>>,
}
