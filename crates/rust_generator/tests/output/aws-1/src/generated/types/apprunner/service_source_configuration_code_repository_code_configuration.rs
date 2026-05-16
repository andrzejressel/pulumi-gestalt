#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfigurationCodeRepositoryCodeConfiguration {
    /// Basic configuration for building and running the App Runner service. Use this parameter to quickly launch an App Runner service without providing an apprunner.yaml file in the source code repository (or ignoring the file if it exists). See Code Configuration Values below for more details.
    #[builder(into)]
    #[serde(rename = "codeConfigurationValues")]
    pub r#code_configuration_values: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues>>,
    /// Source of the App Runner configuration. Valid values: `REPOSITORY`, `API`. Values are interpreted as follows:
    /// * `REPOSITORY` - App Runner reads configuration values from the apprunner.yaml file in the
    /// source code repository and ignores the CodeConfigurationValues parameter.
    /// * `API` - App Runner uses configuration values provided in the CodeConfigurationValues
    /// parameter and ignores the apprunner.yaml file in the source code repository.
    #[builder(into)]
    #[serde(rename = "configurationSource")]
    pub r#configuration_source: String,
}
