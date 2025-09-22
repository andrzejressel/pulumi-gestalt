#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskExecutionStatusLatestJob {
    /// (Output)
    /// The time when the job ended.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// (Output)
    /// Additional information about the current state.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// (Output)
    /// The relative resource name of the job, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/tasks/{taskId}/jobs/{jobId}.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// (Output)
    /// The number of times the job has been retried (excluding the initial attempt).
    #[builder(into)]
    #[serde(rename = "retryCount")]
    pub r#retry_count: Option<i32>,
    /// (Output)
    /// The underlying service running a job.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// (Output)
    /// The full resource name for the job run under a particular service.
    #[builder(into)]
    #[serde(rename = "serviceJob")]
    pub r#service_job: Option<String>,
    /// (Output)
    /// The time when the job was started.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// (Output)
    /// Execution state for the job.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// (Output)
    /// System generated globally unique ID for the job.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Option<String>,
}
