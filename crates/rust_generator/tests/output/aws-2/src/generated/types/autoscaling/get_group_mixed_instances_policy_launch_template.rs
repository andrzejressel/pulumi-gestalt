#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplate {
    /// List of overriding launch template specification objects.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecifications")]
    pub r#launch_template_specifications: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification>,
    /// List of properties overriding the same properties in the launch template.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverride>,
}
