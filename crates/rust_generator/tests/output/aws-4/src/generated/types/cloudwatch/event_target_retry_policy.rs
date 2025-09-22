#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetRetryPolicy {
    /// The age in seconds to continue to make retry attempts.
    #[builder(into)]
    #[serde(rename = "maximumEventAgeInSeconds")]
    pub r#maximum_event_age_in_seconds: Option<i32>,
    /// maximum number of retry attempts to make before the request fails
    #[builder(into)]
    #[serde(rename = "maximumRetryAttempts")]
    pub r#maximum_retry_attempts: Option<i32>,
}
