#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VaultMonitoring {
    /// Enabling/Disabling built-in Azure Monitor alerts for security scenarios and job failure scenarios. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "alertsForAllJobFailuresEnabled")]
    pub r#alerts_for_all_job_failures_enabled: Option<bool>,
    /// Enabling/Disabling alerts from the older (classic alerts) solution. Defaults to `true`. More details could be found [here](https://learn.microsoft.com/en-us/azure/backup/monitoring-and-alerts-overview).
    #[builder(into)]
    #[serde(rename = "alertsForCriticalOperationFailuresEnabled")]
    pub r#alerts_for_critical_operation_failures_enabled: Option<bool>,
}
