#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsMotionGraphicsConfiguration {
    /// Motion Graphics Insertion.
    #[builder(into)]
    #[serde(rename = "motionGraphicsInsertion")]
    pub r#motion_graphics_insertion: Option<String>,
    /// Motion Graphics Settings. See Motion Graphics Settings for more details.
    #[builder(into)]
    #[serde(rename = "motionGraphicsSettings")]
    pub r#motion_graphics_settings: Box<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings>,
}
