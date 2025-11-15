#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification {
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    #[builder(into)]
    #[serde(rename = "allowedInputTypes")]
    pub r#allowed_input_types: Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes>,
    #[builder(into)]
    #[serde(rename = "audioAndDtmfInputSpecification")]
    pub r#audio_and_dtmf_input_specification: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification>>,
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: String,
    #[builder(into)]
    #[serde(rename = "textInputSpecification")]
    pub r#text_input_specification: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationTextInputSpecification>>,
}
