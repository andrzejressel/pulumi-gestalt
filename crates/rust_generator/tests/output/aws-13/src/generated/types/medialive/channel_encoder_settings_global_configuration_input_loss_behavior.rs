#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsGlobalConfigurationInputLossBehavior {
    #[builder(into)]
    #[serde(rename = "blackFrameMsec")]
    pub r#black_frame_msec: Option<i32>,
    #[builder(into)]
    #[serde(rename = "inputLossImageColor")]
    pub r#input_loss_image_color: Option<String>,
    #[builder(into)]
    #[serde(rename = "inputLossImageSlate")]
    pub r#input_loss_image_slate: Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate>>,
    #[builder(into)]
    #[serde(rename = "inputLossImageType")]
    pub r#input_loss_image_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "repeatFrameMsec")]
    pub r#repeat_frame_msec: Option<i32>,
}
