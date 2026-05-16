#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultSpaceSettings {
    /// The settings for assigning a custom file system to a user profile. Permitted users can access this file system in Amazon SageMaker Studio. See `custom_file_system_config` Block below.
    #[builder(into)]
    #[serde(rename = "customFileSystemConfigs")]
    pub r#custom_file_system_configs: Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomFileSystemConfig>>,
    /// Details about the POSIX identity that is used for file system operations. See `custom_posix_user_config` Block below.
    #[builder(into)]
    #[serde(rename = "customPosixUserConfig")]
    pub r#custom_posix_user_config: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomPosixUserConfig>>,
    /// The execution role for the space.
    #[builder(into)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: String,
    /// The settings for the JupyterLab application. See `jupyter_lab_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterLabAppSettings")]
    pub r#jupyter_lab_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See `jupyter_server_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterServerAppSettings")]
    pub r#jupyter_server_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See `kernel_gateway_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "kernelGatewayAppSettings")]
    pub r#kernel_gateway_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsKernelGatewayAppSettings>>,
    /// The security groups for the Amazon Virtual Private Cloud that the space uses for communication.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// The storage settings for a private space. See `space_storage_settings` Block below.
    #[builder(into)]
    #[serde(rename = "spaceStorageSettings")]
    pub r#space_storage_settings: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsSpaceStorageSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultSpaceSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_file_system_configs".to_string(), self.r#custom_file_system_configs.to_pulumi_value().await);
            map.insert("custom_posix_user_config".to_string(), self.r#custom_posix_user_config.to_pulumi_value().await);
            map.insert("execution_role".to_string(), self.r#execution_role.to_pulumi_value().await);
            map.insert("jupyter_lab_app_settings".to_string(), self.r#jupyter_lab_app_settings.to_pulumi_value().await);
            map.insert("jupyter_server_app_settings".to_string(), self.r#jupyter_server_app_settings.to_pulumi_value().await);
            map.insert("kernel_gateway_app_settings".to_string(), self.r#kernel_gateway_app_settings.to_pulumi_value().await);
            map.insert("security_groups".to_string(), self.r#security_groups.to_pulumi_value().await);
            map.insert("space_storage_settings".to_string(), self.r#space_storage_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultSpaceSettings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#custom_file_system_configs: {
                        let field_value = match fields_map.get("custom_file_system_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_file_system_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomFileSystemConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_posix_user_config: {
                        let field_value = match fields_map.get("custom_posix_user_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_posix_user_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomPosixUserConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#execution_role: {
                        let field_value = match fields_map.get("execution_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#jupyter_lab_app_settings: {
                        let field_value = match fields_map.get("jupyter_lab_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'jupyter_lab_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#jupyter_server_app_settings: {
                        let field_value = match fields_map.get("jupyter_server_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'jupyter_server_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterServerAppSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kernel_gateway_app_settings: {
                        let field_value = match fields_map.get("kernel_gateway_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'kernel_gateway_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsKernelGatewayAppSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#space_storage_settings: {
                        let field_value = match fields_map.get("space_storage_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'space_storage_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsSpaceStorageSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
