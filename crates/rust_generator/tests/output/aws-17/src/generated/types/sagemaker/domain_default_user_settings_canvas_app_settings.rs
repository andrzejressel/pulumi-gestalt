#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettingsCanvasAppSettings {
    /// The model deployment settings for the SageMaker Canvas application. See `direct_deploy_settings` Block below.
    #[builder(into)]
    #[serde(rename = "directDeploySettings")]
    pub r#direct_deploy_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsDirectDeploySettings>>,
    /// The settings for running Amazon EMR Serverless jobs in SageMaker Canvas. See `emr_serverless_settings` Block below.
    #[builder(into)]
    #[serde(rename = "emrServerlessSettings")]
    pub r#emr_serverless_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsEmrServerlessSettings>>,
    #[builder(into)]
    #[serde(rename = "generativeAiSettings")]
    pub r#generative_ai_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsGenerativeAiSettings>>,
    /// The settings for connecting to an external data source with OAuth. See `identity_provider_oauth_settings` Block below.
    #[builder(into)]
    #[serde(rename = "identityProviderOauthSettings")]
    pub r#identity_provider_oauth_settings: Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsIdentityProviderOauthSetting>>,
    /// The settings for document querying. See `kendra_settings` Block below.
    #[builder(into)]
    #[serde(rename = "kendraSettings")]
    pub r#kendra_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsKendraSettings>>,
    /// The model registry settings for the SageMaker Canvas application. See `model_register_settings` Block below.
    #[builder(into)]
    #[serde(rename = "modelRegisterSettings")]
    pub r#model_register_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsModelRegisterSettings>>,
    /// Time series forecast settings for the Canvas app. See `time_series_forecasting_settings` Block below.
    #[builder(into)]
    #[serde(rename = "timeSeriesForecastingSettings")]
    pub r#time_series_forecasting_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsTimeSeriesForecastingSettings>>,
    /// The workspace settings for the SageMaker Canvas application. See `workspace_settings` Block below.
    #[builder(into)]
    #[serde(rename = "workspaceSettings")]
    pub r#workspace_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsWorkspaceSettings>>,
}
