#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupVariation {
    #[builder(into, default)]
    #[serde(rename = "customPayloads")]
    pub r#custom_payloads: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupVariationCustomPayload>>>,
    #[builder(into, default)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Box<Option<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupVariationImageResponseCard>>,
    #[builder(into, default)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Box<Option<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupVariationPlainTextMessage>>,
    #[builder(into, default)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Box<Option<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupVariationSsmlMessage>>,
}
