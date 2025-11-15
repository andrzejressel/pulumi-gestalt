#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessage {
    #[builder(into)]
    #[serde(rename = "customPayloads")]
    pub r#custom_payloads: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessageCustomPayload>>,
    #[builder(into)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessageImageResponseCard>>,
    #[builder(into)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessagePlainTextMessage>>,
    #[builder(into)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessageSsmlMessage>>,
}
