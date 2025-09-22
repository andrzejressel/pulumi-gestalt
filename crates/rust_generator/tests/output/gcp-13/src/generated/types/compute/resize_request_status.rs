#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResizeRequestStatus {
    /// (Output)
    /// [Output only] Fatal errors encountered during the queueing or provisioning phases of the ResizeRequest that caused the transition to the FAILED state. Contrary to the lastAttempt errors, this field is final and errors are never removed from here, as the ResizeRequest is not going to retry.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errors")]
    pub r#errors: Option<Vec<super::super::types::compute::ResizeRequestStatusError>>,
    /// (Output)
    /// [Output only] Information about the last attempt to fulfill the request. The value is temporary since the ResizeRequest can retry, as long as it's still active and the last attempt value can either be cleared or replaced with a different error. Since ResizeRequest retries infrequently, the value may be stale and no longer show an active problem. The value is cleared when ResizeRequest transitions to the final state (becomes inactive). If the final state is FAILED the error describing it will be storred in the "error" field only.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lastAttempts")]
    pub r#last_attempts: Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttempt>>,
}
