#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupMixedInstancesPolicy {
    /// List of instances distribution objects.
    #[builder(into)]
    #[serde(rename = "instancesDistributions")]
    pub r#instances_distributions: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyInstancesDistribution>,
    /// List of launch templates along with the overrides.
    #[builder(into)]
    #[serde(rename = "launchTemplates")]
    pub r#launch_templates: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplate>,
}
