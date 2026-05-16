#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelector {
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Option<String>,
    /// Name of the Channel.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "selectorSettings")]
    pub r#selector_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings>>,
}
