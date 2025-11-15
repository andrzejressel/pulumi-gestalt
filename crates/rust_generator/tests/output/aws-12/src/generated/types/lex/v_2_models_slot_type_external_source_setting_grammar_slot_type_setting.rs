#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSetting {
    /// Source of the grammar used to create the slot type.
    /// See `source` argument reference below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<Box<super::super::types::lex::V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSettingSource>>,
}
