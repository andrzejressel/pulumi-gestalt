#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterCoreInstanceFleet {
    /// ID of the cluster.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Configuration block for instance fleet.
    #[builder(into)]
    #[serde(rename = "instanceTypeConfigs")]
    pub r#instance_type_configs: Option<Vec<super::super::types::emr::ClusterCoreInstanceFleetInstanceTypeConfig>>,
    /// Configuration block for launch specification.
    #[builder(into)]
    #[serde(rename = "launchSpecifications")]
    pub r#launch_specifications: Option<Box<super::super::types::emr::ClusterCoreInstanceFleetLaunchSpecifications>>,
    /// Friendly name given to the instance fleet.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "provisionedOnDemandCapacity")]
    pub r#provisioned_on_demand_capacity: Option<i32>,
    #[builder(into)]
    #[serde(rename = "provisionedSpotCapacity")]
    pub r#provisioned_spot_capacity: Option<i32>,
    /// The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision.
    #[builder(into)]
    #[serde(rename = "targetOnDemandCapacity")]
    pub r#target_on_demand_capacity: Option<i32>,
    /// Target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision.
    #[builder(into)]
    #[serde(rename = "targetSpotCapacity")]
    pub r#target_spot_capacity: Option<i32>,
}
