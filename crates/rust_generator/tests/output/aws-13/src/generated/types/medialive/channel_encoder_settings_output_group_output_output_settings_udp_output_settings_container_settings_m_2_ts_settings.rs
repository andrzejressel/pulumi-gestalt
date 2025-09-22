#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettings {
    #[builder(into)]
    #[serde(rename = "absentInputAudioBehavior")]
    pub r#absent_input_audio_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "arib")]
    pub r#arib: Option<String>,
    #[builder(into)]
    #[serde(rename = "aribCaptionsPid")]
    pub r#arib_captions_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "aribCaptionsPidControl")]
    pub r#arib_captions_pid_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioBufferModel")]
    pub r#audio_buffer_model: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioFramesPerPes")]
    pub r#audio_frames_per_pes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "audioPids")]
    pub r#audio_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioStreamType")]
    pub r#audio_stream_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<i32>,
    #[builder(into)]
    #[serde(rename = "bufferModel")]
    pub r#buffer_model: Option<String>,
    #[builder(into)]
    #[serde(rename = "ccDescriptor")]
    pub r#cc_descriptor: Option<String>,
    #[builder(into)]
    #[serde(rename = "dvbNitSettings")]
    pub r#dvb_nit_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbNitSettings>>,
    #[builder(into)]
    #[serde(rename = "dvbSdtSettings")]
    pub r#dvb_sdt_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings>>,
    #[builder(into)]
    #[serde(rename = "dvbSubPids")]
    pub r#dvb_sub_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "dvbTdtSettings")]
    pub r#dvb_tdt_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbTdtSettings>>,
    #[builder(into)]
    #[serde(rename = "dvbTeletextPid")]
    pub r#dvb_teletext_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebif")]
    pub r#ebif: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebpAudioInterval")]
    pub r#ebp_audio_interval: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebpLookaheadMs")]
    pub r#ebp_lookahead_ms: Option<i32>,
    #[builder(into)]
    #[serde(rename = "ebpPlacement")]
    pub r#ebp_placement: Option<String>,
    #[builder(into)]
    #[serde(rename = "ecmPid")]
    pub r#ecm_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "esRateInPes")]
    pub r#es_rate_in_pes: Option<String>,
    #[builder(into)]
    #[serde(rename = "etvPlatformPid")]
    pub r#etv_platform_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "etvSignalPid")]
    pub r#etv_signal_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "fragmentTime")]
    pub r#fragment_time: Option<f64>,
    #[builder(into)]
    #[serde(rename = "klv")]
    pub r#klv: Option<String>,
    #[builder(into)]
    #[serde(rename = "klvDataPids")]
    pub r#klv_data_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "nielsenId3Behavior")]
    pub r#nielsen_id_3_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "nullPacketBitrate")]
    pub r#null_packet_bitrate: Option<f64>,
    #[builder(into)]
    #[serde(rename = "patInterval")]
    pub r#pat_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "pcrControl")]
    pub r#pcr_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "pcrPeriod")]
    pub r#pcr_period: Option<i32>,
    #[builder(into)]
    #[serde(rename = "pcrPid")]
    pub r#pcr_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "pmtInterval")]
    pub r#pmt_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "pmtPid")]
    pub r#pmt_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "programNum")]
    pub r#program_num: Option<i32>,
    #[builder(into)]
    #[serde(rename = "rateMode")]
    pub r#rate_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "scte27Pids")]
    pub r#scte_27_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "scte35Control")]
    pub r#scte_35_control: Option<String>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentationMarkers")]
    pub r#segmentation_markers: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentationStyle")]
    pub r#segmentation_style: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentationTime")]
    pub r#segmentation_time: Option<f64>,
    #[builder(into)]
    #[serde(rename = "timedMetadataBehavior")]
    pub r#timed_metadata_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "timedMetadataPid")]
    pub r#timed_metadata_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "transportStreamId")]
    pub r#transport_stream_id: Option<i32>,
    #[builder(into)]
    #[serde(rename = "videoPid")]
    pub r#video_pid: Option<String>,
}
