#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettings {
    /// Audio descriptions for the channel. See Audio Descriptions for more details.
    #[builder(into)]
    #[serde(rename = "audioDescriptions")]
    pub r#audio_descriptions: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescription>>,
    /// Settings for ad avail blanking. See Avail Blanking for more details.
    #[builder(into)]
    #[serde(rename = "availBlanking")]
    pub r#avail_blanking: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAvailBlanking>>,
    /// Caption Descriptions. See Caption Descriptions for more details.
    #[builder(into)]
    #[serde(rename = "captionDescriptions")]
    pub r#caption_descriptions: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsCaptionDescription>>,
    /// Configuration settings that apply to the event as a whole. See Global Configuration for more details.
    #[builder(into)]
    #[serde(rename = "globalConfiguration")]
    pub r#global_configuration: Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfiguration>>,
    /// Settings for motion graphics. See Motion Graphics Configuration for more details.
    #[builder(into)]
    #[serde(rename = "motionGraphicsConfiguration")]
    pub r#motion_graphics_configuration: Option<Box<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfiguration>>,
    /// Nielsen configuration settings. See Nielsen Configuration for more details.
    #[builder(into)]
    #[serde(rename = "nielsenConfiguration")]
    pub r#nielsen_configuration: Option<Box<super::super::types::medialive::ChannelEncoderSettingsNielsenConfiguration>>,
    /// Output groups for the channel. See Output Groups for more details.
    #[builder(into)]
    #[serde(rename = "outputGroups")]
    pub r#output_groups: Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroup>,
    /// Contains settings used to acquire and adjust timecode information from inputs. See Timecode Config for more details.
    #[builder(into)]
    #[serde(rename = "timecodeConfig")]
    pub r#timecode_config: Box<super::super::types::medialive::ChannelEncoderSettingsTimecodeConfig>,
    /// Video Descriptions. See Video Descriptions for more details.
    #[builder(into)]
    #[serde(rename = "videoDescriptions")]
    pub r#video_descriptions: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsVideoDescription>>,
}
