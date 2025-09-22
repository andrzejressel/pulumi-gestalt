#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobConfigMuxStream {
    /// The container format. The default is `mp4`.
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Option<String>,
    /// List of ElementaryStream.key values multiplexed in this stream.
    #[builder(into)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Option<Vec<String>>,
    /// Identifier of the encryption configuration to use.
    #[builder(into)]
    #[serde(rename = "encryptionId")]
    pub r#encryption_id: Option<String>,
    /// The name of the generated file.
    #[builder(into)]
    #[serde(rename = "fileName")]
    pub r#file_name: Option<String>,
    /// A unique key for this multiplexed stream.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Segment settings for ts, fmp4 and vtt.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "segmentSettings")]
    pub r#segment_settings: Option<Box<super::super::types::transcoder::JobConfigMuxStreamSegmentSettings>>,
}
