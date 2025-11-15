#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainAutoTuneOption {
    /// The Auto-Tune desired state for the domain.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: String,
    /// A list of the nested configurations for the Auto-Tune maintenance windows of the domain.
    #[builder(into)]
    #[serde(rename = "maintenanceSchedules")]
    pub r#maintenance_schedules: Vec<super::super::types::elasticsearch::GetDomainAutoTuneOptionMaintenanceSchedule>,
    /// Whether the domain is set to roll back to default Auto-Tune settings when disabling Auto-Tune.
    #[builder(into)]
    #[serde(rename = "rollbackOnDisable")]
    pub r#rollback_on_disable: String,
}
