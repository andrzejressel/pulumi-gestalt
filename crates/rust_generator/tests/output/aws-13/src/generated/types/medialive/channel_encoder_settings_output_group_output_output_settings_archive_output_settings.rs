#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings {
    /// Settings specific to the container type of the file. See Container Settings for more details.
    #[builder(into)]
    #[serde(rename = "containerSettings")]
    pub r#container_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings>>,
    /// Output file extension.
    #[builder(into)]
    #[serde(rename = "extension")]
    pub r#extension: Option<String>,
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Option<String>,
}
