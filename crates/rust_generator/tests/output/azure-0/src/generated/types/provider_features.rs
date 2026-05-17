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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderFeatures {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "api_management",
                    &self.r#api_management,
                ),
                to_pulumi_object_field(
                    "app_configuration",
                    &self.r#app_configuration,
                ),
                to_pulumi_object_field(
                    "application_insights",
                    &self.r#application_insights,
                ),
                to_pulumi_object_field(
                    "cognitive_account",
                    &self.r#cognitive_account,
                ),
                to_pulumi_object_field(
                    "key_vault",
                    &self.r#key_vault,
                ),
                to_pulumi_object_field(
                    "log_analytics_workspace",
                    &self.r#log_analytics_workspace,
                ),
                to_pulumi_object_field(
                    "machine_learning",
                    &self.r#machine_learning,
                ),
                to_pulumi_object_field(
                    "managed_disk",
                    &self.r#managed_disk,
                ),
                to_pulumi_object_field(
                    "netapp",
                    &self.r#netapp,
                ),
                to_pulumi_object_field(
                    "postgresql_flexible_server",
                    &self.r#postgresql_flexible_server,
                ),
                to_pulumi_object_field(
                    "recovery_service",
                    &self.r#recovery_service,
                ),
                to_pulumi_object_field(
                    "recovery_services_vaults",
                    &self.r#recovery_services_vaults,
                ),
                to_pulumi_object_field(
                    "resource_group",
                    &self.r#resource_group,
                ),
                to_pulumi_object_field(
                    "storage",
                    &self.r#storage,
                ),
                to_pulumi_object_field(
                    "subscription",
                    &self.r#subscription,
                ),
                to_pulumi_object_field(
                    "template_deployment",
                    &self.r#template_deployment,
                ),
                to_pulumi_object_field(
                    "virtual_machine",
                    &self.r#virtual_machine,
                ),
                to_pulumi_object_field(
                    "virtual_machine_scale_set",
                    &self.r#virtual_machine_scale_set,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderFeatures {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#api_management: {
                        let field_value = match fields_map.get("api_management") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_configuration: {
                        let field_value = match fields_map.get("app_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_insights: {
                        let field_value = match fields_map.get("application_insights") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_insights' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cognitive_account: {
                        let field_value = match fields_map.get("cognitive_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognitive_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault: {
                        let field_value = match fields_map.get("key_vault") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_analytics_workspace: {
                        let field_value = match fields_map.get("log_analytics_workspace") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_analytics_workspace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_learning: {
                        let field_value = match fields_map.get("machine_learning") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_learning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk: {
                        let field_value = match fields_map.get("managed_disk") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_disk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#netapp: {
                        let field_value = match fields_map.get("netapp") {
                            Some(value) => value,
                            None => bail!("Missing field 'netapp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postgresql_flexible_server: {
                        let field_value = match fields_map.get("postgresql_flexible_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'postgresql_flexible_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_service: {
                        let field_value = match fields_map.get("recovery_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_services_vaults: {
                        let field_value = match fields_map.get("recovery_services_vaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_services_vaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_group: {
                        let field_value = match fields_map.get("resource_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage: {
                        let field_value = match fields_map.get("storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscription: {
                        let field_value = match fields_map.get("subscription") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscription' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#template_deployment: {
                        let field_value = match fields_map.get("template_deployment") {
                            Some(value) => value,
                            None => bail!("Missing field 'template_deployment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine: {
                        let field_value = match fields_map.get("virtual_machine") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_scale_set: {
                        let field_value = match fields_map.get("virtual_machine_scale_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_scale_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
