#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMonitoringConfig {
    /// Configuration for Advanced Datapath Monitoring. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "advancedDatapathObservabilityConfig")]
    pub r#advanced_datapath_observability_config: Option<Box<super::super::types::container::ClusterMonitoringConfigAdvancedDatapathObservabilityConfig>>,
    /// The GKE components exposing metrics. Supported values include: `SYSTEM_COMPONENTS`, `APISERVER`, `SCHEDULER`, `CONTROLLER_MANAGER`, `STORAGE`, `HPA`, `POD`, `DAEMONSET`, `DEPLOYMENT`, `STATEFULSET`, `KUBELET`, `CADVISOR` and `DCGM`. In beta provider, `WORKLOADS` is supported on top of those 12 values. (`WORKLOADS` is deprecated and removed in GKE 1.24.) `KUBELET` and `CADVISOR` are only supported in GKE 1.29.3-gke.1093000 and above.
    #[builder(into)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Option<Vec<String>>,
    /// Configuration for Managed Service for Prometheus. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "managedPrometheus")]
    pub r#managed_prometheus: Option<Box<super::super::types::container::ClusterMonitoringConfigManagedPrometheus>>,
}
