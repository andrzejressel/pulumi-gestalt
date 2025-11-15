#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpotFleetRequestLaunchTemplateConfigOverride {
    /// The availability zone in which to place the request.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    /// The instance requirements. See below.
    #[builder(into)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Option<Box<super::super::types::ec2::SpotFleetRequestLaunchTemplateConfigOverrideInstanceRequirements>>,
    /// The type of instance to request.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// The priority for the launch template override. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
    /// The maximum spot bid for this override request.
    #[builder(into)]
    #[serde(rename = "spotPrice")]
    pub r#spot_price: Option<String>,
    /// The subnet in which to launch the requested instance.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// The capacity added to the fleet by a fulfilled request.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Option<f64>,
}
