#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification {
    /// Configuration block for the settings on audio input. See `audio_specification`.
    #[builder(into)]
    #[serde(rename = "audioSpecification")]
    pub r#audio_specification: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationAudioSpecification>>,
    /// Configuration block for the settings on DTMF input. See `dtmf_specification`.
    #[builder(into)]
    #[serde(rename = "dtmfSpecification")]
    pub r#dtmf_specification: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification>>,
    /// Time for which a bot waits before assuming that the customer isn't going to speak or press a key. This timeout is shared between Audio and DTMF inputs.
    #[builder(into)]
    #[serde(rename = "startTimeoutMs")]
    pub r#start_timeout_ms: i32,
}
