#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterDefaultNodePoolKubeletConfig {
    /// Specifies the allow list of unsafe sysctls command or patterns (ending in `*`).
    #[builder(into)]
    #[serde(rename = "allowedUnsafeSysctls")]
    pub r#allowed_unsafe_sysctls: Option<Vec<String>>,
    /// Specifies the maximum number of container log files that can be present for a container. must be at least 2.
    #[builder(into)]
    #[serde(rename = "containerLogMaxLine")]
    pub r#container_log_max_line: Option<i32>,
    /// Specifies the maximum size (e.g. 10MB) of container log file before it is rotated.
    #[builder(into)]
    #[serde(rename = "containerLogMaxSizeMb")]
    pub r#container_log_max_size_mb: Option<i32>,
    /// Is CPU CFS quota enforcement for containers enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaEnabled")]
    pub r#cpu_cfs_quota_enabled: Option<bool>,
    /// Specifies the CPU CFS quota period value.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Option<String>,
    /// Specifies the CPU Manager policy to use. Possible values are `none` and `static`,.
    #[builder(into)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Option<String>,
    /// Specifies the percent of disk usage above which image garbage collection is always run. Must be between `0` and `100`.
    #[builder(into)]
    #[serde(rename = "imageGcHighThreshold")]
    pub r#image_gc_high_threshold: Option<i32>,
    /// Specifies the percent of disk usage lower than which image garbage collection is never run. Must be between `0` and `100`.
    #[builder(into)]
    #[serde(rename = "imageGcLowThreshold")]
    pub r#image_gc_low_threshold: Option<i32>,
    /// Specifies the maximum number of processes per pod.
    #[builder(into)]
    #[serde(rename = "podMaxPid")]
    pub r#pod_max_pid: Option<i32>,
    /// Specifies the Topology Manager policy to use. Possible values are `none`, `best-effort`, `restricted` or `single-numa-node`.
    #[builder(into)]
    #[serde(rename = "topologyManagerPolicy")]
    pub r#topology_manager_policy: Option<String>,
}
