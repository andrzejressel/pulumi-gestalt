#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkloadComplianceStatus {
    /// Number of current orgPolicy violations which are acknowledged.
    #[builder(into)]
    #[serde(rename = "acknowledgedViolationCounts")]
    pub r#acknowledged_violation_counts: Option<Vec<i32>>,
    /// Number of current orgPolicy violations which are not acknowledged.
    #[builder(into)]
    #[serde(rename = "activeViolationCounts")]
    pub r#active_violation_counts: Option<Vec<i32>>,
}
