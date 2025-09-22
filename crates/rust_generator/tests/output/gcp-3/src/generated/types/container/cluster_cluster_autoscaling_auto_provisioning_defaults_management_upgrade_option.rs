#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaultsManagementUpgradeOption {
    /// This field is set when upgrades are about to commence with the approximate start time for the upgrades, in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "autoUpgradeStartTime")]
    pub r#auto_upgrade_start_time: Option<String>,
    /// Description of the cluster.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}
