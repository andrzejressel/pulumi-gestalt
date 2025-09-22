#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroup {
    /// Custom output group name defined by the user.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Settings associated with the output group. See Output Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "outputGroupSettings")]
    pub r#output_group_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettings>,
    /// List of outputs. See Outputs for more details.
    #[builder(into)]
    #[serde(rename = "outputs")]
    pub r#outputs: Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutput>,
}
