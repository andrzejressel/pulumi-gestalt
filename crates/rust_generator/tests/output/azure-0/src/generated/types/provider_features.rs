#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeatures {
    #[builder(into)]
    #[serde(rename = "apiManagement")]
    pub r#api_management: Option<Box<super::types::ProviderFeaturesApiManagement>>,
    #[builder(into)]
    #[serde(rename = "appConfiguration")]
    pub r#app_configuration: Option<Box<super::types::ProviderFeaturesAppConfiguration>>,
    #[builder(into)]
    #[serde(rename = "applicationInsights")]
    pub r#application_insights: Option<Box<super::types::ProviderFeaturesApplicationInsights>>,
    #[builder(into)]
    #[serde(rename = "cognitiveAccount")]
    pub r#cognitive_account: Option<Box<super::types::ProviderFeaturesCognitiveAccount>>,
    #[builder(into)]
    #[serde(rename = "keyVault")]
    pub r#key_vault: Option<Box<super::types::ProviderFeaturesKeyVault>>,
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspace")]
    pub r#log_analytics_workspace: Option<Box<super::types::ProviderFeaturesLogAnalyticsWorkspace>>,
    #[builder(into)]
    #[serde(rename = "machineLearning")]
    pub r#machine_learning: Option<Box<super::types::ProviderFeaturesMachineLearning>>,
    #[builder(into)]
    #[serde(rename = "managedDisk")]
    pub r#managed_disk: Option<Box<super::types::ProviderFeaturesManagedDisk>>,
    #[builder(into)]
    #[serde(rename = "netapp")]
    pub r#netapp: Option<Box<super::types::ProviderFeaturesNetapp>>,
    #[builder(into)]
    #[serde(rename = "postgresqlFlexibleServer")]
    pub r#postgresql_flexible_server: Option<Box<super::types::ProviderFeaturesPostgresqlFlexibleServer>>,
    #[builder(into)]
    #[serde(rename = "recoveryService")]
    pub r#recovery_service: Option<Box<super::types::ProviderFeaturesRecoveryService>>,
    #[builder(into)]
    #[serde(rename = "recoveryServicesVaults")]
    pub r#recovery_services_vaults: Option<Box<super::types::ProviderFeaturesRecoveryServicesVaults>>,
    #[builder(into)]
    #[serde(rename = "resourceGroup")]
    pub r#resource_group: Option<Box<super::types::ProviderFeaturesResourceGroup>>,
    #[builder(into)]
    #[serde(rename = "storage")]
    pub r#storage: Option<Box<super::types::ProviderFeaturesStorage>>,
    #[builder(into)]
    #[serde(rename = "subscription")]
    pub r#subscription: Option<Box<super::types::ProviderFeaturesSubscription>>,
    #[builder(into)]
    #[serde(rename = "templateDeployment")]
    pub r#template_deployment: Option<Box<super::types::ProviderFeaturesTemplateDeployment>>,
    #[builder(into)]
    #[serde(rename = "virtualMachine")]
    pub r#virtual_machine: Option<Box<super::types::ProviderFeaturesVirtualMachine>>,
    #[builder(into)]
    #[serde(rename = "virtualMachineScaleSet")]
    pub r#virtual_machine_scale_set: Option<Box<super::types::ProviderFeaturesVirtualMachineScaleSet>>,
}
