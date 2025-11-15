#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolPlacementPolicy {
    /// If set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned.
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Option<String>,
    /// TPU placement topology for pod slice node pool. https://cloud.google.com/tpu/docs/types-topologies#tpu_topologies
    #[builder(into)]
    #[serde(rename = "tpuTopology")]
    pub r#tpu_topology: Option<String>,
    /// Telemetry integration for the cluster. Supported values (`ENABLED, DISABLED, SYSTEM_ONLY`);
    /// `SYSTEM_ONLY` (Only system components are monitored and logged) is only available in GKE versions 1.15 and later.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
