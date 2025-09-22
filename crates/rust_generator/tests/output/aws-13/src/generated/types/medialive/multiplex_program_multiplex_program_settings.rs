#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MultiplexProgramMultiplexProgramSettings {
    /// Enum for preferred channel pipeline. Options are `CURRENTLY_ACTIVE`, `PIPELINE_0`, or `PIPELINE_1`.
    #[builder(into)]
    #[serde(rename = "preferredChannelPipeline")]
    pub r#preferred_channel_pipeline: String,
    /// Unique program number.
    #[builder(into)]
    #[serde(rename = "programNumber")]
    pub r#program_number: i32,
    /// Service Descriptor. See Service Descriptor for more details.
    #[builder(into)]
    #[serde(rename = "serviceDescriptor")]
    pub r#service_descriptor: Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsServiceDescriptor>>,
    /// Video settings. See Video Settings for more details.
    #[builder(into)]
    #[serde(rename = "videoSettings")]
    pub r#video_settings: Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettings>>,
}
