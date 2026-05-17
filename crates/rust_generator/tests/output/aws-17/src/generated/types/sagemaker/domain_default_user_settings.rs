#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettings {
    /// Indicates whether auto-mounting of an EFS volume is supported for the user profile. The `DefaultAsDomain` value is only supported for user profiles. Do not use the `DefaultAsDomain` value when setting this parameter for a domain. Valid values are: `Enabled`, `Disabled`, and `DefaultAsDomain`.
    #[builder(into)]
    #[serde(rename = "autoMountHomeEfs")]
    pub r#auto_mount_home_efs: Option<String>,
    /// The Canvas app settings. See `canvas_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "canvasAppSettings")]
    pub r#canvas_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettings>>,
    /// The Code Editor application settings. See `code_editor_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "codeEditorAppSettings")]
    pub r#code_editor_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCodeEditorAppSettings>>,
    /// The settings for assigning a custom file system to a user profile. Permitted users can access this file system in Amazon SageMaker Studio. See `custom_file_system_config` Block below.
    #[builder(into)]
    #[serde(rename = "customFileSystemConfigs")]
    pub r#custom_file_system_configs: Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsCustomFileSystemConfig>>,
    /// Details about the POSIX identity that is used for file system operations. See `custom_posix_user_config` Block below.
    #[builder(into)]
    #[serde(rename = "customPosixUserConfig")]
    pub r#custom_posix_user_config: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCustomPosixUserConfig>>,
    /// The default experience that the user is directed to when accessing the domain. The supported values are: `studio::`: Indicates that Studio is the default experience. This value can only be passed if StudioWebPortal is set to ENABLED. `app:JupyterServer:`: Indicates that Studio Classic is the default experience.
    #[builder(into)]
    #[serde(rename = "defaultLandingUri")]
    pub r#default_landing_uri: Option<String>,
    /// The execution role ARN for the user.
    #[builder(into)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: String,
    /// The settings for the JupyterLab application. See `jupyter_lab_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterLabAppSettings")]
    pub r#jupyter_lab_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See `jupyter_server_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterServerAppSettings")]
    pub r#jupyter_server_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See `kernel_gateway_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "kernelGatewayAppSettings")]
    pub r#kernel_gateway_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsKernelGatewayAppSettings>>,
    /// The RSession app settings. See `r_session_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "rSessionAppSettings")]
    pub r#r_session_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettings>>,
    /// A collection of settings that configure user interaction with the RStudioServerPro app. See `r_studio_server_pro_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "rStudioServerProAppSettings")]
    pub r#r_studio_server_pro_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsRStudioServerProAppSettings>>,
    /// A list of security group IDs that will be attached to the user.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// The sharing settings. See `sharing_settings` Block below.
    #[builder(into)]
    #[serde(rename = "sharingSettings")]
    pub r#sharing_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsSharingSettings>>,
    /// The storage settings for a private space. See `space_storage_settings` Block below.
    #[builder(into)]
    #[serde(rename = "spaceStorageSettings")]
    pub r#space_storage_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsSpaceStorageSettings>>,
    /// Whether the user can access Studio. If this value is set to `DISABLED`, the user cannot access Studio, even if that is the default experience for the domain. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "studioWebPortal")]
    pub r#studio_web_portal: Option<String>,
    /// The Studio Web Portal settings. See `studio_web_portal_settings` Block below.
    #[builder(into)]
    #[serde(rename = "studioWebPortalSettings")]
    pub r#studio_web_portal_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsStudioWebPortalSettings>>,
    /// The TensorBoard app settings. See `tensor_board_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "tensorBoardAppSettings")]
    pub r#tensor_board_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsTensorBoardAppSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultUserSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "auto_mount_home_efs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_mount_home_efs,
                )
                .await,
            );
            map.insert(
                "canvas_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#canvas_app_settings,
                )
                .await,
            );
            map.insert(
                "code_editor_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#code_editor_app_settings,
                )
                .await,
            );
            map.insert(
                "custom_file_system_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_file_system_configs,
                )
                .await,
            );
            map.insert(
                "custom_posix_user_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_posix_user_config,
                )
                .await,
            );
            map.insert(
                "default_landing_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_landing_uri,
                )
                .await,
            );
            map.insert(
                "execution_role".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_role,
                )
                .await,
            );
            map.insert(
                "jupyter_lab_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jupyter_lab_app_settings,
                )
                .await,
            );
            map.insert(
                "jupyter_server_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jupyter_server_app_settings,
                )
                .await,
            );
            map.insert(
                "kernel_gateway_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kernel_gateway_app_settings,
                )
                .await,
            );
            map.insert(
                "r_session_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#r_session_app_settings,
                )
                .await,
            );
            map.insert(
                "r_studio_server_pro_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#r_studio_server_pro_app_settings,
                )
                .await,
            );
            map.insert(
                "security_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_groups,
                )
                .await,
            );
            map.insert(
                "sharing_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sharing_settings,
                )
                .await,
            );
            map.insert(
                "space_storage_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#space_storage_settings,
                )
                .await,
            );
            map.insert(
                "studio_web_portal".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#studio_web_portal,
                )
                .await,
            );
            map.insert(
                "studio_web_portal_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#studio_web_portal_settings,
                )
                .await,
            );
            map.insert(
                "tensor_board_app_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tensor_board_app_settings,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultUserSettings {
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
                    r#auto_mount_home_efs: {
                        let field_value = match fields_map.get("auto_mount_home_efs") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_mount_home_efs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#canvas_app_settings: {
                        let field_value = match fields_map.get("canvas_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'canvas_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_editor_app_settings: {
                        let field_value = match fields_map.get("code_editor_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_editor_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#default_landing_uri: {
                        let field_value = match fields_map.get("default_landing_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_landing_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#r_session_app_settings: {
                        let field_value = match fields_map.get("r_session_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_session_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_studio_server_pro_app_settings: {
                        let field_value = match fields_map.get("r_studio_server_pro_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_studio_server_pro_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sharing_settings: {
                        let field_value = match fields_map.get("sharing_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'sharing_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#studio_web_portal: {
                        let field_value = match fields_map.get("studio_web_portal") {
                            Some(value) => value,
                            None => bail!("Missing field 'studio_web_portal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#studio_web_portal_settings: {
                        let field_value = match fields_map.get("studio_web_portal_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'studio_web_portal_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tensor_board_app_settings: {
                        let field_value = match fields_map.get("tensor_board_app_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'tensor_board_app_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
