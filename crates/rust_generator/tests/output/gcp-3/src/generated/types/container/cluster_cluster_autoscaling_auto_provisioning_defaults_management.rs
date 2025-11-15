#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaultsManagement {
    /// Specifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered.
    /// 
    /// This block also contains several computed attributes, documented below.
    #[builder(into)]
    #[serde(rename = "autoRepair")]
    pub r#auto_repair: Option<bool>,
    /// Specifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes.
    #[builder(into)]
    #[serde(rename = "autoUpgrade")]
    pub r#auto_upgrade: Option<bool>,
    /// Specifies the [Auto Upgrade knobs](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/NodeManagement#AutoUpgradeOptions) for the node pool.
    #[builder(into)]
    #[serde(rename = "upgradeOptions")]
    pub r#upgrade_options: Option<Vec<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsManagementUpgradeOption>>,
}
