#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsBotLocaleVoiceSettings {
    /// Indicates the type of Amazon Polly voice that Amazon Lex should use for voice interaction with the user. Valid values are `standard` and `neural`. If not specified, the default is `standard`.
    #[builder(into)]
    #[serde(rename = "engine")]
    pub r#engine: Option<String>,
    /// Identifier of the Amazon Polly voice to use.
    #[builder(into)]
    #[serde(rename = "voiceId")]
    pub r#voice_id: String,
}
