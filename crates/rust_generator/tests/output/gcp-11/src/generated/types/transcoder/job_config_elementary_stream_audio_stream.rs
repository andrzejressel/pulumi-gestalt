#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobConfigElementaryStreamAudioStream {
    /// Audio bitrate in bits per second.
    #[builder(into)]
    #[serde(rename = "bitrateBps")]
    pub r#bitrate_bps: i32,
    /// Number of audio channels. The default is `2`.
    #[builder(into)]
    #[serde(rename = "channelCount")]
    pub r#channel_count: Option<i32>,
    /// A list of channel names specifying layout of the audio channels. The default is ["fl", "fr"].
    #[builder(into)]
    #[serde(rename = "channelLayouts")]
    pub r#channel_layouts: Option<Vec<String>>,
    /// The codec for this audio stream. The default is `aac`.
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: Option<String>,
    /// The audio sample rate in Hertz. The default is `48000`.
    #[builder(into)]
    #[serde(rename = "sampleRateHertz")]
    pub r#sample_rate_hertz: Option<i32>,
}
