#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings {
    #[builder(into)]
    #[serde(rename = "h265PackagingType")]
    pub r#h_265_packaging_type: Option<String>,
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Option<String>,
}
