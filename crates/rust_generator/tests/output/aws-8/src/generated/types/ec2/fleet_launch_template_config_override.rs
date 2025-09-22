#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetLaunchTemplateConfigOverride {
    /// Availability Zone in which to launch the instances.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    /// Override the instance type in the Launch Template with instance types that satisfy the requirements.
    #[builder(into)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Option<Box<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirements>>,
    /// Instance type.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Maximum price per unit hour that you are willing to pay for a Spot Instance.
    #[builder(into)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: Option<String>,
    /// Priority for the launch template override. If `on_demand_options` `allocation_strategy` is set to `prioritized`, EC2 Fleet uses priority to determine which launch template override to use first in fulfilling On-Demand capacity. The highest priority is launched first. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority. Valid values are whole numbers starting at 0.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
    /// ID of the subnet in which to launch the instances.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// Number of units provided by the specified instance type.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Option<f64>,
}
