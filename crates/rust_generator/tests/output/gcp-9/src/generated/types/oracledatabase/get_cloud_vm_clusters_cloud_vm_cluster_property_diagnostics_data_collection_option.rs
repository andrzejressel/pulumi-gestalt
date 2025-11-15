#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudVmClustersCloudVmClusterPropertyDiagnosticsDataCollectionOption {
    /// Indicates whether diagnostic collection is enabled for the VM cluster
    #[builder(into)]
    #[serde(rename = "diagnosticsEventsEnabled")]
    pub r#diagnostics_events_enabled: bool,
    /// Indicates whether health monitoring is enabled for the VM cluster
    #[builder(into)]
    #[serde(rename = "healthMonitoringEnabled")]
    pub r#health_monitoring_enabled: bool,
    /// Indicates whether incident logs and trace collection are enabled for the VM
    /// cluster
    #[builder(into)]
    #[serde(rename = "incidentLogsEnabled")]
    pub r#incident_logs_enabled: bool,
}
