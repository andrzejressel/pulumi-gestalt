#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
    #[builder(into)]
    #[serde(rename = "acquisitionPointId")]
    pub r#acquisition_point_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioOnlyTimecodeControl")]
    pub r#audio_only_timecode_control: Option<String>,
    /// Setting to allow self signed or verified RTMP certificates.
    #[builder(into)]
    #[serde(rename = "certificateMode")]
    pub r#certificate_mode: Option<String>,
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination>,
    #[builder(into)]
    #[serde(rename = "eventId")]
    pub r#event_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "eventIdMode")]
    pub r#event_id_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "eventStopBehavior")]
    pub r#event_stop_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Option<i32>,
    #[builder(into)]
    #[serde(rename = "fragmentLength")]
    pub r#fragment_length: Option<i32>,
    #[builder(into)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Option<String>,
    /// Number of retry attempts.
    #[builder(into)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Option<i32>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Option<i32>,
    #[builder(into)]
    #[serde(rename = "segmentationMode")]
    pub r#segmentation_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "sendDelayMs")]
    pub r#send_delay_ms: Option<i32>,
    #[builder(into)]
    #[serde(rename = "sparseTrackType")]
    pub r#sparse_track_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "streamManifestBehavior")]
    pub r#stream_manifest_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "timestampOffset")]
    pub r#timestamp_offset: Option<String>,
    #[builder(into)]
    #[serde(rename = "timestampOffsetMode")]
    pub r#timestamp_offset_mode: Option<String>,
}
