#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupAvailabilityZoneDistribution {
    /// The strategy to use for distributing capacity across the Availability Zones. Valid values are `balanced-only` and `balanced-best-effort`. Default is `balanced-best-effort`.
    #[builder(into)]
    #[serde(rename = "capacityDistributionStrategy")]
    pub r#capacity_distribution_strategy: Option<String>,
}
