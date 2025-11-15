#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetTargetCapacitySpecification {
    /// Default target capacity type. Valid values: `on-demand`, `spot`.
    #[builder(into)]
    #[serde(rename = "defaultTargetCapacityType")]
    pub r#default_target_capacity_type: String,
    /// The number of On-Demand units to request.
    #[builder(into)]
    #[serde(rename = "onDemandTargetCapacity")]
    pub r#on_demand_target_capacity: Option<i32>,
    /// The number of Spot units to request.
    #[builder(into)]
    #[serde(rename = "spotTargetCapacity")]
    pub r#spot_target_capacity: Option<i32>,
    /// The unit for the target capacity.
    /// If you specify `target_capacity_unit_type`, `instance_requirements` must be specified.
    #[builder(into)]
    #[serde(rename = "targetCapacityUnitType")]
    pub r#target_capacity_unit_type: Option<String>,
    /// The number of units to request, filled using `default_target_capacity_type`.
    #[builder(into)]
    #[serde(rename = "totalTargetCapacity")]
    pub r#total_target_capacity: i32,
}
