#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings {
    /// The frequency at which to capture frames for inclusion in the output.
    #[builder(into)]
    #[serde(rename = "captureInterval")]
    pub r#capture_interval: Option<i32>,
    /// Unit for the frame capture interval.
    #[builder(into)]
    #[serde(rename = "captureIntervalUnits")]
    pub r#capture_interval_units: Option<String>,
}
