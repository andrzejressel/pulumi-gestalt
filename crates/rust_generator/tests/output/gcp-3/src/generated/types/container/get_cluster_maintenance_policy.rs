#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMaintenancePolicy {
    /// Time window specified for daily maintenance operations. Specify start_time in RFC3339 format "HH:MM”, where HH : [00-23] and MM : [00-59] GMT.
    #[builder(into)]
    #[serde(rename = "dailyMaintenanceWindows")]
    pub r#daily_maintenance_windows: Vec<super::super::types::container::GetClusterMaintenancePolicyDailyMaintenanceWindow>,
    /// Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows.
    #[builder(into)]
    #[serde(rename = "maintenanceExclusions")]
    pub r#maintenance_exclusions: Vec<super::super::types::container::GetClusterMaintenancePolicyMaintenanceExclusion>,
    /// Time window for recurring maintenance operations.
    #[builder(into)]
    #[serde(rename = "recurringWindows")]
    pub r#recurring_windows: Vec<super::super::types::container::GetClusterMaintenancePolicyRecurringWindow>,
}
