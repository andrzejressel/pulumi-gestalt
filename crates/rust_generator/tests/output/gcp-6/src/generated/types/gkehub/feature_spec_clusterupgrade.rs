#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureSpecClusterupgrade {
    /// Configuration overrides for individual upgrades.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gkeUpgradeOverrides")]
    pub r#gke_upgrade_overrides: Option<Vec<super::super::types::gkehub::FeatureSpecClusterupgradeGkeUpgradeOverride>>,
    /// Post conditions to override for the specified upgrade.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postConditions")]
    pub r#post_conditions: Option<Box<super::super::types::gkehub::FeatureSpecClusterupgradePostConditions>>,
    /// Specified if other fleet should be considered as a source of upgrades. Currently, at most one upstream fleet is allowed. The fleet name should be either fleet project number or id.
    #[builder(into)]
    #[serde(rename = "upstreamFleets")]
    pub r#upstream_fleets: Vec<String>,
}
