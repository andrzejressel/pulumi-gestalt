#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobRetryConfig {
    /// The maximum amount of time to wait before retrying a job after it fails.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    #[builder(into)]
    #[serde(rename = "maxBackoffDuration")]
    pub r#max_backoff_duration: Option<String>,
    /// The time between retries will double maxDoublings times.
    /// A job's retry interval starts at minBackoffDuration,
    /// then doubles maxDoublings times, then increases linearly,
    /// and finally retries retries at intervals of maxBackoffDuration up to retryCount times.
    #[builder(into)]
    #[serde(rename = "maxDoublings")]
    pub r#max_doublings: Option<i32>,
    /// The time limit for retrying a failed job, measured from time when an execution was first attempted.
    /// If specified with retryCount, the job will be retried until both limits are reached.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    #[builder(into)]
    #[serde(rename = "maxRetryDuration")]
    pub r#max_retry_duration: Option<String>,
    /// The minimum amount of time to wait before retrying a job after it fails.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    #[builder(into)]
    #[serde(rename = "minBackoffDuration")]
    pub r#min_backoff_duration: Option<String>,
    /// The number of attempts that the system will make to run a
    /// job using the exponential backoff procedure described by maxDoublings.
    /// Values greater than 5 and negative values are not allowed.
    #[builder(into)]
    #[serde(rename = "retryCount")]
    pub r#retry_count: Option<i32>,
}
