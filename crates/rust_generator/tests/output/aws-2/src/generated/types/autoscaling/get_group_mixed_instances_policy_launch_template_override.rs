#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverride {
    /// List of instance requirements objects.
    /// * `accelerator_count - List of objects describing the minimum and maximum number of accelerators for an instance type.
    #[builder(into)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement>,
    /// Overriding instance type.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    /// List of overriding launch template specification objects.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecifications")]
    pub r#launch_template_specifications: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification>,
    /// Number of capacity units, which gives the instance type a proportional weight to other instance types.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: String,
}
