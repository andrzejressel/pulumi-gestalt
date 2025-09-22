#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterAddonsConfigRayOperatorConfig {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The status of Ray Logging, which scrapes Ray cluster logs to Cloud Logging. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "rayClusterLoggingConfig")]
    pub r#ray_cluster_logging_config: Box<Option<super::super::types::container::ClusterAddonsConfigRayOperatorConfigRayClusterLoggingConfig>>,
    /// The status of Ray Cluster monitoring, which shows Ray cluster metrics in Cloud Console. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "rayClusterMonitoringConfig")]
    pub r#ray_cluster_monitoring_config: Box<Option<super::super::types::container::ClusterAddonsConfigRayOperatorConfigRayClusterMonitoringConfig>>,
}
