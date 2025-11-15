#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings {
    /// Specifies behavior of last resort when input video os lost.
    #[builder(into)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Option<String>,
    /// Indicates ID3 frame that has the timecode.
    #[builder(into)]
    #[serde(rename = "timedMetadataId3Frame")]
    pub r#timed_metadata_id_3_frame: Option<String>,
    #[builder(into)]
    #[serde(rename = "timedMetadataId3Period")]
    pub r#timed_metadata_id_3_period: Option<i32>,
}
