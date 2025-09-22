#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterProtectConfig {
    /// WorkloadConfig defines which actions are enabled for a cluster's workload configurations.
    #[builder(into)]
    #[serde(rename = "workloadConfigs")]
    pub r#workload_configs: Vec<super::super::types::container::GetClusterProtectConfigWorkloadConfig>,
    /// Sets which mode to use for Protect workload vulnerability scanning feature. Accepted values are DISABLED, BASIC.
    #[builder(into)]
    #[serde(rename = "workloadVulnerabilityMode")]
    pub r#workload_vulnerability_mode: String,
}
