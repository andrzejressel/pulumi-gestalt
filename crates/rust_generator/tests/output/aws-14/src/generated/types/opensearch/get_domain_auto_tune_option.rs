#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainAutoTuneOption {
    /// Auto-Tune desired state for the domain.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: String,
    /// A list of the nested configurations for the Auto-Tune maintenance windows of the domain.
    #[builder(into)]
    #[serde(rename = "maintenanceSchedules")]
    pub r#maintenance_schedules: Vec<super::super::types::opensearch::GetDomainAutoTuneOptionMaintenanceSchedule>,
    /// Whether the domain is set to roll back to default Auto-Tune settings when disabling Auto-Tune.
    #[builder(into)]
    #[serde(rename = "rollbackOnDisable")]
    pub r#rollback_on_disable: String,
    /// Whether to schedule Auto-Tune optimizations that require blue/green deployments during the domain's configured daily off-peak window.
    #[builder(into)]
    #[serde(rename = "useOffPeakWindow")]
    pub r#use_off_peak_window: bool,
}
