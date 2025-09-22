#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettings {
    /// Archive output settings. See Archive Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "archiveOutputSettings")]
    pub r#archive_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings>>,
    #[builder(into)]
    #[serde(rename = "frameCaptureOutputSettings")]
    pub r#frame_capture_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsOutputSettings")]
    pub r#hls_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings>>,
    /// Media package output settings. This can be set as an empty block.
    #[builder(into)]
    #[serde(rename = "mediaPackageOutputSettings")]
    pub r#media_package_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings>>,
    #[builder(into)]
    #[serde(rename = "msSmoothOutputSettings")]
    pub r#ms_smooth_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings>>,
    /// Multiplex output settings. See Multiplex Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "multiplexOutputSettings")]
    pub r#multiplex_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings>>,
    /// RTMP output settings. See RTMP Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "rtmpOutputSettings")]
    pub r#rtmp_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings>>,
    /// UDP output settings. See UDP Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "udpOutputSettings")]
    pub r#udp_output_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings>>,
}
