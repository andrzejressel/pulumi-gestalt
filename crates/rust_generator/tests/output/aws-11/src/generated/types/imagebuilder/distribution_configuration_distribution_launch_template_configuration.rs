#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionLaunchTemplateConfiguration {
    /// The account ID that this configuration applies to.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// Indicates whether to set the specified Amazon EC2 launch template as the default launch template. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "default")]
    pub r#default: Option<bool>,
    /// The ID of the Amazon EC2 launch template to use.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: String,
}
