#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateConfig {
    /// Ad break.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "adBreaks")]
    pub r#ad_breaks: Option<Vec<super::super::types::transcoder::JobTemplateConfigAdBreak>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "editLists")]
    pub r#edit_lists: Option<Vec<super::super::types::transcoder::JobTemplateConfigEditList>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Option<Vec<super::super::types::transcoder::JobTemplateConfigElementaryStream>>,
    /// List of encryption configurations for the content.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptions")]
    pub r#encryptions: Option<Vec<super::super::types::transcoder::JobTemplateConfigEncryption>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "inputs")]
    pub r#inputs: Option<Vec<super::super::types::transcoder::JobTemplateConfigInput>>,
    /// Manifest configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "manifests")]
    pub r#manifests: Option<Vec<super::super::types::transcoder::JobTemplateConfigManifest>>,
    /// Multiplexing settings for output stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "muxStreams")]
    pub r#mux_streams: Option<Vec<super::super::types::transcoder::JobTemplateConfigMuxStream>>,
    /// Location of output file(s) in a Cloud Storage bucket.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "output")]
    pub r#output: Option<Box<super::super::types::transcoder::JobTemplateConfigOutput>>,
    /// List of overlays on the output video, in descending Z-order.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "overlays")]
    pub r#overlays: Option<Vec<super::super::types::transcoder::JobTemplateConfigOverlay>>,
    /// Pub/Sub destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubsubDestination")]
    pub r#pubsub_destination: Option<Box<super::super::types::transcoder::JobTemplateConfigPubsubDestination>>,
}
