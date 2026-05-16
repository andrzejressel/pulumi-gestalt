#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping {
    #[builder(into)]
    #[serde(rename = "inputChannelLevels")]
    pub r#input_channel_levels: Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel>,
    #[builder(into)]
    #[serde(rename = "outputChannel")]
    pub r#output_channel: i32,
}
