#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification {
    /// ID of the launch template. Conflicts with `launch_template_name`.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Option<String>,
    /// Name of the launch template. Conflicts with `launch_template_id`.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
