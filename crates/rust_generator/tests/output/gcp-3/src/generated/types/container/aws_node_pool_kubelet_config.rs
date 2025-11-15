#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsNodePoolKubeletConfig {
    /// Whether or not to enable CPU CFS quota. Defaults to true.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: Option<bool>,
    /// Optional. The CPU CFS quota period to use for the node. Defaults to "100ms".
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Option<String>,
    /// The CpuManagerPolicy to use for the node. Defaults to "none".
    #[builder(into)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Option<String>,
    /// Optional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset.
    #[builder(into)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: Option<i32>,
}
