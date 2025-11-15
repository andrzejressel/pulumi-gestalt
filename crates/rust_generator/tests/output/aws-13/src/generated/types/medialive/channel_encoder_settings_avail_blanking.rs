#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAvailBlanking {
    /// Blanking image to be used. See Avail Blanking Image for more details.
    #[builder(into)]
    #[serde(rename = "availBlankingImage")]
    pub r#avail_blanking_image: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAvailBlankingAvailBlankingImage>>,
    /// When set to enabled, causes video, audio and captions to be blanked when insertion metadata is added.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
