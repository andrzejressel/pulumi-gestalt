#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceSourceConfiguration {
    /// Describes resources needed to authenticate access to some source repositories. See Authentication Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "authenticationConfiguration")]
    pub r#authentication_configuration: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationAuthenticationConfiguration>>,
    /// Whether continuous integration from the source repository is enabled for the App Runner service. If set to `true`, each repository change (source code commit or new image version) starts a deployment. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoDeploymentsEnabled")]
    pub r#auto_deployments_enabled: Option<bool>,
    /// Description of a source code repository. See Code Repository below for more details.
    #[builder(into)]
    #[serde(rename = "codeRepository")]
    pub r#code_repository: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepository>>,
    /// Description of a source image repository. See Image Repository below for more details.
    #[builder(into)]
    #[serde(rename = "imageRepository")]
    pub r#image_repository: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationImageRepository>>,
}
