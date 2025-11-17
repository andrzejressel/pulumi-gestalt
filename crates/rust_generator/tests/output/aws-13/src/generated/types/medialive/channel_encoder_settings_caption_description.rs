#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsCaptionDescription {
    /// Indicates whether the caption track implements accessibility features such as written descriptions of spoken dialog, music, and sounds.
    #[builder(into)]
    #[serde(rename = "accessibility")]
    pub r#accessibility: Option<String>,
    /// Specifies which input caption selector to use as a caption source when generating output captions. This field should match a captionSelector name.
    #[builder(into)]
    #[serde(rename = "captionSelectorName")]
    pub r#caption_selector_name: String,
    /// Additional settings for captions destination that depend on the destination type. See Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "destinationSettings")]
    pub r#destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettings>>,
    /// ISO 639-2 three-digit code.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Option<String>,
    /// Human readable information to indicate captions available for players (eg. English, or Spanish).
    #[builder(into)]
    #[serde(rename = "languageDescription")]
    pub r#language_description: Option<String>,
    /// Name of the caption description. Used to associate a caption description with an output. Names must be unique within an event.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
