#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterProtectConfig {
    /// WorkloadConfig defines which actions are enabled for a cluster's workload configurations. Structure is documented below
    #[builder(into)]
    #[serde(rename = "workloadConfig")]
    pub r#workload_config: Option<Box<super::super::types::container::ClusterProtectConfigWorkloadConfig>>,
    /// Sets which mode to use for Protect workload vulnerability scanning feature. Accepted values are DISABLED, BASIC.
    #[builder(into)]
    #[serde(rename = "workloadVulnerabilityMode")]
    pub r#workload_vulnerability_mode: Option<String>,
}
