#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanExecutionStatus {
    /// (Output)
    /// The time when the latest DataScanJob started.
    #[builder(into)]
    #[serde(rename = "latestJobEndTime")]
    pub r#latest_job_end_time: Option<String>,
    /// (Output)
    /// The time when the latest DataScanJob ended.
    #[builder(into)]
    #[serde(rename = "latestJobStartTime")]
    pub r#latest_job_start_time: Option<String>,
}
