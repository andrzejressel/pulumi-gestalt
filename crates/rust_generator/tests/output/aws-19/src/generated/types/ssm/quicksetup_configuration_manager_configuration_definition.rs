#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QuicksetupConfigurationManagerConfigurationDefinition {
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[builder(into)]
    #[serde(rename = "localDeploymentAdministrationRoleArn")]
    pub r#local_deployment_administration_role_arn: Option<String>,
    /// Name of the IAM role used to deploy local configurations.
    #[builder(into)]
    #[serde(rename = "localDeploymentExecutionRoleName")]
    pub r#local_deployment_execution_role_name: Option<String>,
    /// Parameters for the configuration definition type. Parameters for configuration definitions vary based the configuration type. See the [AWS API documentation](https://docs.aws.amazon.com/quick-setup/latest/APIReference/API_ConfigurationDefinitionInput.html) for a complete list of parameters for each configuration type.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: std::collections::HashMap<String, String>,
    /// Type of the Quick Setup configuration.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Version of the Quick Setup type to use.
    #[builder(into)]
    #[serde(rename = "typeVersion")]
    pub r#type_version: Option<String>,
}
