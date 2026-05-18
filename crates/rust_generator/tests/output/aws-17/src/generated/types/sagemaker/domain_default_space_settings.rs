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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "custom_file_system_configs",
                    &self.r#custom_file_system_configs,
                ),
                to_pulumi_object_field(
                    "custom_posix_user_config",
                    &self.r#custom_posix_user_config,
                ),
                to_pulumi_object_field(
                    "execution_role",
                    &self.r#execution_role,
                ),
                to_pulumi_object_field(
                    "jupyter_lab_app_settings",
                    &self.r#jupyter_lab_app_settings,
                ),
                to_pulumi_object_field(
                    "jupyter_server_app_settings",
                    &self.r#jupyter_server_app_settings,
                ),
                to_pulumi_object_field(
                    "kernel_gateway_app_settings",
                    &self.r#kernel_gateway_app_settings,
                ),
                to_pulumi_object_field(
                    "security_groups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "space_storage_settings",
                    &self.r#space_storage_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultSpaceSettings {
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
                    r#custom_file_system_configs: {
                        let field_value = match fields_map.get("custom_file_system_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_file_system_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_posix_user_config: {
                        let field_value = match fields_map.get("custom_posix_user_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_posix_user_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role: {
                        let field_value = match fields_map.get("execution_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jupyter_lab_app_settings: {
                        let field_value = match fields_map.get("jupyter_lab_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'jupyter_lab_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jupyter_server_app_settings: {
                        let field_value = match fields_map.get("jupyter_server_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'jupyter_server_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kernel_gateway_app_settings: {
                        let field_value = match fields_map.get("kernel_gateway_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'kernel_gateway_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#space_storage_settings: {
                        let field_value = match fields_map.get("space_storage_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'space_storage_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
