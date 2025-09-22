#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolUpgradeSettings {
    /// Settings for blue-green upgrade strategy. To be specified when strategy is set to BLUE_GREEN. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Box<Option<super::super::types::container::ClusterNodePoolUpgradeSettingsBlueGreenSettings>>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process. To be used when strategy is set to SURGE. Default is 0.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Option<i32>,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process. To be used when strategy is set to SURGE. Default is 0.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Option<i32>,
    /// Strategy used for node pool update. Strategy can only be one of BLUE_GREEN or SURGE. The default is value is SURGE.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: Option<String>,
}
