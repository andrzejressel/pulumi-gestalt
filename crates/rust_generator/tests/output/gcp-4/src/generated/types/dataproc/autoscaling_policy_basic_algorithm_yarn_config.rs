#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscalingPolicyBasicAlgorithmYarnConfig {
    /// Timeout for YARN graceful decommissioning of Node Managers. Specifies the
    /// duration to wait for jobs to complete before forcefully removing workers
    /// (and potentially interrupting jobs). Only applicable to downscaling operations.
    /// Bounds: [0s, 1d].
    #[builder(into)]
    #[serde(rename = "gracefulDecommissionTimeout")]
    pub r#graceful_decommission_timeout: Box<String>,
    /// Fraction of average pending memory in the last cooldown period for which to
    /// remove workers. A scale-down factor of 1 will result in scaling down so that there
    /// is no available memory remaining after the update (more aggressive scaling).
    /// A scale-down factor of 0 disables removing workers, which can be beneficial for
    /// autoscaling a single job.
    /// Bounds: [0.0, 1.0].
    #[builder(into)]
    #[serde(rename = "scaleDownFactor")]
    pub r#scale_down_factor: Box<f64>,
    /// Minimum scale-down threshold as a fraction of total cluster size before scaling occurs.
    /// For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must
    /// recommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0
    /// means the autoscaler will scale down on any recommended change.
    /// Bounds: [0.0, 1.0]. Default: 0.0.
    #[builder(into, default)]
    #[serde(rename = "scaleDownMinWorkerFraction")]
    pub r#scale_down_min_worker_fraction: Box<Option<f64>>,
    /// Fraction of average pending memory in the last cooldown period for which to
    /// add workers. A scale-up factor of 1.0 will result in scaling up so that there
    /// is no pending memory remaining after the update (more aggressive scaling).
    /// A scale-up factor closer to 0 will result in a smaller magnitude of scaling up
    /// (less aggressive scaling).
    /// Bounds: [0.0, 1.0].
    #[builder(into)]
    #[serde(rename = "scaleUpFactor")]
    pub r#scale_up_factor: Box<f64>,
    /// Minimum scale-up threshold as a fraction of total cluster size before scaling
    /// occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler
    /// must recommend at least a 2-worker scale-up for the cluster to scale. A threshold of
    /// 0 means the autoscaler will scale up on any recommended change.
    /// Bounds: [0.0, 1.0]. Default: 0.0.
    #[builder(into, default)]
    #[serde(rename = "scaleUpMinWorkerFraction")]
    pub r#scale_up_min_worker_fraction: Box<Option<f64>>,
}
