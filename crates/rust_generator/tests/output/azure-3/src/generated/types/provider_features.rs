#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeatures {
    #[builder(into)]
    pub r#api_management: Option<Box<super::types::ProviderFeaturesApiManagement>>,
    #[builder(into)]
    pub r#app_configuration: Option<Box<super::types::ProviderFeaturesAppConfiguration>>,
    #[builder(into)]
    pub r#application_insights: Option<Box<super::types::ProviderFeaturesApplicationInsights>>,
    #[builder(into)]
    pub r#cognitive_account: Option<Box<super::types::ProviderFeaturesCognitiveAccount>>,
    #[builder(into)]
    pub r#key_vault: Option<Box<super::types::ProviderFeaturesKeyVault>>,
    #[builder(into)]
    pub r#log_analytics_workspace: Option<Box<super::types::ProviderFeaturesLogAnalyticsWorkspace>>,
    #[builder(into)]
    pub r#machine_learning: Option<Box<super::types::ProviderFeaturesMachineLearning>>,
    #[builder(into)]
    pub r#managed_disk: Option<Box<super::types::ProviderFeaturesManagedDisk>>,
    #[builder(into)]
    pub r#netapp: Option<Box<super::types::ProviderFeaturesNetapp>>,
    #[builder(into)]
    pub r#postgresql_flexible_server: Option<Box<super::types::ProviderFeaturesPostgresqlFlexibleServer>>,
    #[builder(into)]
    pub r#recovery_service: Option<Box<super::types::ProviderFeaturesRecoveryService>>,
    #[builder(into)]
    pub r#recovery_services_vaults: Option<Box<super::types::ProviderFeaturesRecoveryServicesVaults>>,
    #[builder(into)]
    pub r#resource_group: Option<Box<super::types::ProviderFeaturesResourceGroup>>,
    #[builder(into)]
    pub r#storage: Option<Box<super::types::ProviderFeaturesStorage>>,
    #[builder(into)]
    pub r#subscription: Option<Box<super::types::ProviderFeaturesSubscription>>,
    #[builder(into)]
    pub r#template_deployment: Option<Box<super::types::ProviderFeaturesTemplateDeployment>>,
    #[builder(into)]
    pub r#virtual_machine: Option<Box<super::types::ProviderFeaturesVirtualMachine>>,
    #[builder(into)]
    pub r#virtual_machine_scale_set: Option<Box<super::types::ProviderFeaturesVirtualMachineScaleSet>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderFeatures {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "apiManagement",
                    &self.r#api_management,
                ),
                to_pulumi_object_field(
                    "appConfiguration",
                    &self.r#app_configuration,
                ),
                to_pulumi_object_field(
                    "applicationInsights",
                    &self.r#application_insights,
                ),
                to_pulumi_object_field(
                    "cognitiveAccount",
                    &self.r#cognitive_account,
                ),
                to_pulumi_object_field(
                    "keyVault",
                    &self.r#key_vault,
                ),
                to_pulumi_object_field(
                    "logAnalyticsWorkspace",
                    &self.r#log_analytics_workspace,
                ),
                to_pulumi_object_field(
                    "machineLearning",
                    &self.r#machine_learning,
                ),
                to_pulumi_object_field(
                    "managedDisk",
                    &self.r#managed_disk,
                ),
                to_pulumi_object_field(
                    "netapp",
                    &self.r#netapp,
                ),
                to_pulumi_object_field(
                    "postgresqlFlexibleServer",
                    &self.r#postgresql_flexible_server,
                ),
                to_pulumi_object_field(
                    "recoveryService",
                    &self.r#recovery_service,
                ),
                to_pulumi_object_field(
                    "recoveryServicesVaults",
                    &self.r#recovery_services_vaults,
                ),
                to_pulumi_object_field(
                    "resourceGroup",
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
                    "templateDeployment",
                    &self.r#template_deployment,
                ),
                to_pulumi_object_field(
                    "virtualMachine",
                    &self.r#virtual_machine,
                ),
                to_pulumi_object_field(
                    "virtualMachineScaleSet",
                    &self.r#virtual_machine_scale_set,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("apiManagement") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiManagement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_configuration: {
                        let field_value = match fields_map.get("appConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'appConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_insights: {
                        let field_value = match fields_map.get("applicationInsights") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationInsights' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cognitive_account: {
                        let field_value = match fields_map.get("cognitiveAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognitiveAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault: {
                        let field_value = match fields_map.get("keyVault") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyVault' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_analytics_workspace: {
                        let field_value = match fields_map.get("logAnalyticsWorkspace") {
                            Some(value) => value,
                            None => bail!("Missing field 'logAnalyticsWorkspace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_learning: {
                        let field_value = match fields_map.get("machineLearning") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineLearning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk: {
                        let field_value = match fields_map.get("managedDisk") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedDisk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("postgresqlFlexibleServer") {
                            Some(value) => value,
                            None => bail!("Missing field 'postgresqlFlexibleServer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_service: {
                        let field_value = match fields_map.get("recoveryService") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoveryService' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_services_vaults: {
                        let field_value = match fields_map.get("recoveryServicesVaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoveryServicesVaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_group: {
                        let field_value = match fields_map.get("resourceGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("templateDeployment") {
                            Some(value) => value,
                            None => bail!("Missing field 'templateDeployment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine: {
                        let field_value = match fields_map.get("virtualMachine") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualMachine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_scale_set: {
                        let field_value = match fields_map.get("virtualMachineScaleSet") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualMachineScaleSet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
