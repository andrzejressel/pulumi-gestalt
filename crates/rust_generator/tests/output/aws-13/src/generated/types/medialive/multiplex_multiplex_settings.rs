#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MultiplexMultiplexSettings {
    /// Maximum video buffer delay.
    #[builder(into)]
    #[serde(rename = "maximumVideoBufferDelayMilliseconds")]
    pub r#maximum_video_buffer_delay_milliseconds: Option<i32>,
    /// Transport stream bit rate.
    #[builder(into)]
    #[serde(rename = "transportStreamBitrate")]
    pub r#transport_stream_bitrate: i32,
    /// Unique ID for each multiplex.
    #[builder(into)]
    #[serde(rename = "transportStreamId")]
    pub r#transport_stream_id: i32,
    /// Transport stream reserved bit rate.
    #[builder(into)]
    #[serde(rename = "transportStreamReservedBitrate")]
    pub r#transport_stream_reserved_bitrate: Option<i32>,
}
