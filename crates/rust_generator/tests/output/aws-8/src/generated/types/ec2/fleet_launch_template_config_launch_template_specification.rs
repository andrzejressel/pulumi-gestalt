#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetLaunchTemplateConfigLaunchTemplateSpecification {
    /// The ID of the launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Option<String>,
    /// The name of the launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Option<String>,
    /// The launch template version number, `$Latest`, or `$Default.`
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
