#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CaCertificateRegistrationConfig {
    /// The ARN of the role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
    /// The template body.
    #[builder(into)]
    #[serde(rename = "templateBody")]
    pub r#template_body: Option<String>,
    /// The name of the provisioning template.
    #[builder(into)]
    #[serde(rename = "templateName")]
    pub r#template_name: Option<String>,
}
