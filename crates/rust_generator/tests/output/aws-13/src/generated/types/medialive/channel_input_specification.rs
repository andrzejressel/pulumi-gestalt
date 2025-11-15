#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputSpecification {
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: String,
    #[builder(into)]
    #[serde(rename = "inputResolution")]
    pub r#input_resolution: String,
    #[builder(into)]
    #[serde(rename = "maximumBitrate")]
    pub r#maximum_bitrate: String,
}
