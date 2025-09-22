#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettings {
    /// Archive group settings. See Archive Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "archiveGroupSettings")]
    pub r#archive_group_settings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting>>,
    #[builder(into)]
    #[serde(rename = "frameCaptureGroupSettings")]
    pub r#frame_capture_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsGroupSettings")]
    pub r#hls_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings>>,
    /// Media package group settings. See Media Package Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "mediaPackageGroupSettings")]
    pub r#media_package_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "msSmoothGroupSettings")]
    pub r#ms_smooth_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "multiplexGroupSettings")]
    pub r#multiplex_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings>>,
    /// RTMP group settings. See RTMP Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "rtmpGroupSettings")]
    pub r#rtmp_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "udpGroupSettings")]
    pub r#udp_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings>>,
}
