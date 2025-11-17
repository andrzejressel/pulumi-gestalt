#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterAutoScalerProfile {
    /// Detect similar node groups and balance the number of nodes between them. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "balanceSimilarNodeGroups")]
    pub r#balance_similar_node_groups: Option<bool>,
    /// Whether DaemonSet pods will be gracefully terminated from empty nodes. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "daemonsetEvictionForEmptyNodesEnabled")]
    pub r#daemonset_eviction_for_empty_nodes_enabled: Option<bool>,
    /// Whether DaemonSet pods will be gracefully terminated from non-empty nodes. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "daemonsetEvictionForOccupiedNodesEnabled")]
    pub r#daemonset_eviction_for_occupied_nodes_enabled: Option<bool>,
    /// Maximum number of empty nodes that can be deleted at the same time. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "emptyBulkDeleteMax")]
    pub r#empty_bulk_delete_max: Option<String>,
    /// Expander to use. Possible values are `least-waste`, `priority`, `most-pods` and `random`. Defaults to `random`.
    #[builder(into)]
    #[serde(rename = "expander")]
    pub r#expander: Option<String>,
    /// Whether DaemonSet pods will be ignored when calculating resource utilization for scale down. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ignoreDaemonsetsUtilizationEnabled")]
    pub r#ignore_daemonsets_utilization_enabled: Option<bool>,
    /// Maximum number of seconds the cluster autoscaler waits for pod termination when trying to scale down a node. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "maxGracefulTerminationSec")]
    pub r#max_graceful_termination_sec: Option<String>,
    /// Maximum time the autoscaler waits for a node to be provisioned. Defaults to `15m`.
    #[builder(into)]
    #[serde(rename = "maxNodeProvisioningTime")]
    pub r#max_node_provisioning_time: Option<String>,
    /// Maximum Number of allowed unready nodes. Defaults to `3`.
    #[builder(into)]
    #[serde(rename = "maxUnreadyNodes")]
    pub r#max_unready_nodes: Option<i32>,
    /// Maximum percentage of unready nodes the cluster autoscaler will stop if the percentage is exceeded. Defaults to `45`.
    #[builder(into)]
    #[serde(rename = "maxUnreadyPercentage")]
    pub r#max_unready_percentage: Option<f64>,
    /// For scenarios like burst/batch scale where you don't want CA to act before the kubernetes scheduler could schedule all the pods, you can tell CA to ignore unscheduled pods before they're a certain age. Defaults to `10s`.
    #[builder(into)]
    #[serde(rename = "newPodScaleUpDelay")]
    pub r#new_pod_scale_up_delay: Option<String>,
    /// How long after the scale up of AKS nodes the scale down evaluation resumes. Defaults to `10m`.
    #[builder(into)]
    #[serde(rename = "scaleDownDelayAfterAdd")]
    pub r#scale_down_delay_after_add: Option<String>,
    /// How long after node deletion that scale down evaluation resumes. Defaults to the value used for `scan_interval`.
    #[builder(into)]
    #[serde(rename = "scaleDownDelayAfterDelete")]
    pub r#scale_down_delay_after_delete: Option<String>,
    /// How long after scale down failure that scale down evaluation resumes. Defaults to `3m`.
    #[builder(into)]
    #[serde(rename = "scaleDownDelayAfterFailure")]
    pub r#scale_down_delay_after_failure: Option<String>,
    /// How long a node should be unneeded before it is eligible for scale down. Defaults to `10m`.
    #[builder(into)]
    #[serde(rename = "scaleDownUnneeded")]
    pub r#scale_down_unneeded: Option<String>,
    /// How long an unready node should be unneeded before it is eligible for scale down. Defaults to `20m`.
    #[builder(into)]
    #[serde(rename = "scaleDownUnready")]
    pub r#scale_down_unready: Option<String>,
    /// Node utilization level, defined as sum of requested resources divided by capacity, below which a node can be considered for scale down. Defaults to `0.5`.
    #[builder(into)]
    #[serde(rename = "scaleDownUtilizationThreshold")]
    pub r#scale_down_utilization_threshold: Option<String>,
    /// How often the AKS Cluster should be re-evaluated for scale up/down. Defaults to `10s`.
    #[builder(into)]
    #[serde(rename = "scanInterval")]
    pub r#scan_interval: Option<String>,
    /// If `true` cluster autoscaler will never delete nodes with pods with local storage, for example, EmptyDir or HostPath. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "skipNodesWithLocalStorage")]
    pub r#skip_nodes_with_local_storage: Option<bool>,
    /// If `true` cluster autoscaler will never delete nodes with pods from kube-system (except for DaemonSet or mirror pods). Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "skipNodesWithSystemPods")]
    pub r#skip_nodes_with_system_pods: Option<bool>,
}
