#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxAgentSpeechToTextSettings {
    /// Whether to use speech adaptation for speech recognition.
    #[builder(into)]
    #[serde(rename = "enableSpeechAdaptation")]
    pub r#enable_speech_adaptation: Option<bool>,
}
