#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsTimecodeConfig {
    /// The source for the timecode that will be associated with the events outputs.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
    /// Threshold in frames beyond which output timecode is resynchronized to the input timecode.
    #[builder(into)]
    #[serde(rename = "syncThreshold")]
    pub r#sync_threshold: Option<i32>,
}
