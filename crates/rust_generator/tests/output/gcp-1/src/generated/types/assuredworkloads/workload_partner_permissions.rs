#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkloadPartnerPermissions {
    /// Optional. Allow partner to view violation alerts.
    #[builder(into)]
    #[serde(rename = "assuredWorkloadsMonitoring")]
    pub r#assured_workloads_monitoring: Option<bool>,
    /// Allow the partner to view inspectability logs and monitoring violations.
    #[builder(into)]
    #[serde(rename = "dataLogsViewer")]
    pub r#data_logs_viewer: Option<bool>,
    /// Optional. Allow partner to view access approval logs.
    #[builder(into)]
    #[serde(rename = "serviceAccessApprover")]
    pub r#service_access_approver: Option<bool>,
}
