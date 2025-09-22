#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification {
    /// ID of the launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: String,
    /// Name of the launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: String,
    /// Template version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
