#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecorderRecordingMode {
    /// Default reecording frequency. `CONTINUOUS` or `DAILY`.
    #[builder(into)]
    #[serde(rename = "recordingFrequency")]
    pub r#recording_frequency: Option<String>,
    /// Recording mode overrides. Detailed below.
    #[builder(into)]
    #[serde(rename = "recordingModeOverride")]
    pub r#recording_mode_override: Option<Box<super::super::types::cfg::RecorderRecordingModeRecordingModeOverride>>,
}
