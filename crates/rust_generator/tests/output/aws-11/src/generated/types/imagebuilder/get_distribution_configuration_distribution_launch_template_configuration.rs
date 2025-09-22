#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDistributionConfigurationDistributionLaunchTemplateConfiguration {
    /// The account ID that this configuration applies to.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// Whether the specified Amazon EC2 launch template is set as the default launch template.
    #[builder(into)]
    #[serde(rename = "default")]
    pub r#default: bool,
    /// ID of the Amazon EC2 launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: String,
}
