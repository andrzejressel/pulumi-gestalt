#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetAudio {
    /// The method of organizing audio channels and tracks. Use Audio:Channels to specify the number of channels in your output, and Audio:AudioPackingMode to specify the number of tracks and their relation to the channels. If you do not specify an Audio:AudioPackingMode, Elastic Transcoder uses SingleTrack.
    #[builder(into)]
    #[serde(rename = "audioPackingMode")]
    pub r#audio_packing_mode: Option<String>,
    /// The bit rate of the audio stream in the output file, in kilobits/second. Enter an integer between 64 and 320, inclusive.
    #[builder(into)]
    #[serde(rename = "bitRate")]
    pub r#bit_rate: Option<String>,
    /// The number of audio channels in the output file
    #[builder(into)]
    #[serde(rename = "channels")]
    pub r#channels: Option<String>,
    /// The audio codec for the output file. Valid values are `AAC`, `flac`, `mp2`, `mp3`, `pcm`, and `vorbis`.
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: Option<String>,
    /// The sample rate of the audio stream in the output file, in hertz. Valid values are: `auto`, `22050`, `32000`, `44100`, `48000`, `96000`
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<String>,
}
