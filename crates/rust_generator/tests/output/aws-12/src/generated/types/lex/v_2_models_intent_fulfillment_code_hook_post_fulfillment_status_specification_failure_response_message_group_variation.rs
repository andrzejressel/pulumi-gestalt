#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureResponseMessageGroupVariation {
    /// Configuration block for a message in a custom format defined by the client application. See `custom_payload`.
    #[builder(into)]
    #[serde(rename = "customPayload")]
    pub r#custom_payload: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureResponseMessageGroupVariationCustomPayload>>,
    /// Configuration block for a message that defines a response card that the client application can show to the user. See `image_response_card`.
    #[builder(into)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureResponseMessageGroupVariationImageResponseCard>>,
    /// Configuration block for a message in plain text format. See `plain_text_message`.
    #[builder(into)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureResponseMessageGroupVariationPlainTextMessage>>,
    /// Configuration block for a message in Speech Synthesis Markup Language (SSML). See `ssml_message`.
    #[builder(into)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureResponseMessageGroupVariationSsmlMessage>>,
}
