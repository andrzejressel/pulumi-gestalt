#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecorderRecordingMode {
    /// Default reecording frequency. `CONTINUOUS` or `DAILY`.
    #[builder(into)]
    #[serde(rename = "recordingFrequency")]
    pub r#recording_frequency: Option<String>,
    /// Recording mode overrides. Detailed below.
    #[builder(into)]
    #[serde(rename = "recordingModeOverride")]
    pub r#recording_mode_override: Box<Option<super::super::types::cfg::RecorderRecordingModeRecordingModeOverride>>,
}
