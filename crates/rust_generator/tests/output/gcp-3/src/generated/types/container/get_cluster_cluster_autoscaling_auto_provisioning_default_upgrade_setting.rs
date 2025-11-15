#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSetting {
    /// Settings for blue-green upgrade strategy.
    #[builder(into)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSettingBlueGreenSetting>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: i32,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: i32,
    /// Update strategy of the node pool.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: String,
}
