#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetLaunchTemplateConfig {
    /// Nested argument containing EC2 Launch Template to use. Defined below.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecification")]
    pub r#launch_template_specification: Option<Box<super::super::types::ec2::FleetLaunchTemplateConfigLaunchTemplateSpecification>>,
    /// Nested argument(s) containing parameters to override the same parameters in the Launch Template. Defined below.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Option<Vec<super::super::types::ec2::FleetLaunchTemplateConfigOverride>>,
}
