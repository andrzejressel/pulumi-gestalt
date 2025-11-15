#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTerminalCondition {
    /// A reason for the execution condition.
    #[builder(into)]
    #[serde(rename = "executionReason")]
    pub r#execution_reason: String,
    /// Last time the condition transitioned from one status to another.
    #[builder(into)]
    #[serde(rename = "lastTransitionTime")]
    pub r#last_transition_time: String,
    /// Human readable message indicating details about the current status.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: String,
    /// A common (service-level) reason for this condition.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: String,
    /// A reason for the revision condition.
    #[builder(into)]
    #[serde(rename = "revisionReason")]
    pub r#revision_reason: String,
    /// How to interpret failures of this condition, one of Error, Warning, Info
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: String,
    /// State of the condition.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// type is used to communicate the status of the reconciliation process. See also: https://github.com/knative/serving/blob/main/docs/spec/errors.md#error-conditions-and-reporting Types common to all resources include: * "Ready": True when the Resource is ready.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
