#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordingConfigurationThumbnailConfiguration {
    /// Thumbnail recording mode. Valid values: `DISABLED`, `INTERVAL`.
    #[builder(into)]
    #[serde(rename = "recordingMode")]
    pub r#recording_mode: Option<String>,
    /// The targeted thumbnail-generation interval in seconds.
    #[builder(into)]
    #[serde(rename = "targetIntervalSeconds")]
    pub r#target_interval_seconds: Option<i32>,
}
