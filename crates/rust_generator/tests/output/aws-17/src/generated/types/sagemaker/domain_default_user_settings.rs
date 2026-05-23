#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettings {
    /// Indicates whether auto-mounting of an EFS volume is supported for the user profile. The `DefaultAsDomain` value is only supported for user profiles. Do not use the `DefaultAsDomain` value when setting this parameter for a domain. Valid values are: `Enabled`, `Disabled`, and `DefaultAsDomain`.
    #[builder(into)]
    pub r#auto_mount_home_efs: Option<String>,
    /// The Canvas app settings. See `canvas_app_settings` Block below.
    #[builder(into)]
    pub r#canvas_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettings>>,
    /// The Code Editor application settings. See `code_editor_app_settings` Block below.
    #[builder(into)]
    pub r#code_editor_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCodeEditorAppSettings>>,
    /// The settings for assigning a custom file system to a user profile. Permitted users can access this file system in Amazon SageMaker Studio. See `custom_file_system_config` Block below.
    #[builder(into)]
    pub r#custom_file_system_configs: Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsCustomFileSystemConfig>>,
    /// Details about the POSIX identity that is used for file system operations. See `custom_posix_user_config` Block below.
    #[builder(into)]
    pub r#custom_posix_user_config: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCustomPosixUserConfig>>,
    /// The default experience that the user is directed to when accessing the domain. The supported values are: `studio::`: Indicates that Studio is the default experience. This value can only be passed if StudioWebPortal is set to ENABLED. `app:JupyterServer:`: Indicates that Studio Classic is the default experience.
    #[builder(into)]
    pub r#default_landing_uri: Option<String>,
    /// The execution role ARN for the user.
    #[builder(into)]
    pub r#execution_role: String,
    /// The settings for the JupyterLab application. See `jupyter_lab_app_settings` Block below.
    #[builder(into)]
    pub r#jupyter_lab_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See `jupyter_server_app_settings` Block below.
    #[builder(into)]
    pub r#jupyter_server_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See `kernel_gateway_app_settings` Block below.
    #[builder(into)]
    pub r#kernel_gateway_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsKernelGatewayAppSettings>>,
    /// The RSession app settings. See `r_session_app_settings` Block below.
    #[builder(into)]
    pub r#r_session_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettings>>,
    /// A collection of settings that configure user interaction with the RStudioServerPro app. See `r_studio_server_pro_app_settings` Block below.
    #[builder(into)]
    pub r#r_studio_server_pro_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsRStudioServerProAppSettings>>,
    /// A list of security group IDs that will be attached to the user.
    #[builder(into)]
    pub r#security_groups: Option<Vec<String>>,
    /// The sharing settings. See `sharing_settings` Block below.
    #[builder(into)]
    pub r#sharing_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsSharingSettings>>,
    /// The storage settings for a private space. See `space_storage_settings` Block below.
    #[builder(into)]
    pub r#space_storage_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsSpaceStorageSettings>>,
    /// Whether the user can access Studio. If this value is set to `DISABLED`, the user cannot access Studio, even if that is the default experience for the domain. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    pub r#studio_web_portal: Option<String>,
    /// The Studio Web Portal settings. See `studio_web_portal_settings` Block below.
    #[builder(into)]
    pub r#studio_web_portal_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsStudioWebPortalSettings>>,
    /// The TensorBoard app settings. See `tensor_board_app_settings` Block below.
    #[builder(into)]
    pub r#tensor_board_app_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsTensorBoardAppSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultUserSettings {
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
                    "autoMountHomeEfs",
                    &self.r#auto_mount_home_efs,
                ),
                to_pulumi_object_field(
                    "canvasAppSettings",
                    &self.r#canvas_app_settings,
                ),
                to_pulumi_object_field(
                    "codeEditorAppSettings",
                    &self.r#code_editor_app_settings,
                ),
                to_pulumi_object_field(
                    "customFileSystemConfigs",
                    &self.r#custom_file_system_configs,
                ),
                to_pulumi_object_field(
                    "customPosixUserConfig",
                    &self.r#custom_posix_user_config,
                ),
                to_pulumi_object_field(
                    "defaultLandingUri",
                    &self.r#default_landing_uri,
                ),
                to_pulumi_object_field(
                    "executionRole",
                    &self.r#execution_role,
                ),
                to_pulumi_object_field(
                    "jupyterLabAppSettings",
                    &self.r#jupyter_lab_app_settings,
                ),
                to_pulumi_object_field(
                    "jupyterServerAppSettings",
                    &self.r#jupyter_server_app_settings,
                ),
                to_pulumi_object_field(
                    "kernelGatewayAppSettings",
                    &self.r#kernel_gateway_app_settings,
                ),
                to_pulumi_object_field(
                    "rSessionAppSettings",
                    &self.r#r_session_app_settings,
                ),
                to_pulumi_object_field(
                    "rStudioServerProAppSettings",
                    &self.r#r_studio_server_pro_app_settings,
                ),
                to_pulumi_object_field(
                    "securityGroups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "sharingSettings",
                    &self.r#sharing_settings,
                ),
                to_pulumi_object_field(
                    "spaceStorageSettings",
                    &self.r#space_storage_settings,
                ),
                to_pulumi_object_field(
                    "studioWebPortal",
                    &self.r#studio_web_portal,
                ),
                to_pulumi_object_field(
                    "studioWebPortalSettings",
                    &self.r#studio_web_portal_settings,
                ),
                to_pulumi_object_field(
                    "tensorBoardAppSettings",
                    &self.r#tensor_board_app_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("autoMountHomeEfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoMountHomeEfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#canvas_app_settings: {
                        let field_value = match fields_map.get("canvasAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'canvasAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_editor_app_settings: {
                        let field_value = match fields_map.get("codeEditorAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeEditorAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_file_system_configs: {
                        let field_value = match fields_map.get("customFileSystemConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'customFileSystemConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_posix_user_config: {
                        let field_value = match fields_map.get("customPosixUserConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'customPosixUserConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_landing_uri: {
                        let field_value = match fields_map.get("defaultLandingUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultLandingUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role: {
                        let field_value = match fields_map.get("executionRole") {
                            Some(value) => value,
                            None => bail!("Missing field 'executionRole' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jupyter_lab_app_settings: {
                        let field_value = match fields_map.get("jupyterLabAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'jupyterLabAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jupyter_server_app_settings: {
                        let field_value = match fields_map.get("jupyterServerAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'jupyterServerAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kernel_gateway_app_settings: {
                        let field_value = match fields_map.get("kernelGatewayAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'kernelGatewayAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_session_app_settings: {
                        let field_value = match fields_map.get("rSessionAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rSessionAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_studio_server_pro_app_settings: {
                        let field_value = match fields_map.get("rStudioServerProAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rStudioServerProAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("securityGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sharing_settings: {
                        let field_value = match fields_map.get("sharingSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'sharingSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#space_storage_settings: {
                        let field_value = match fields_map.get("spaceStorageSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'spaceStorageSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#studio_web_portal: {
                        let field_value = match fields_map.get("studioWebPortal") {
                            Some(value) => value,
                            None => bail!("Missing field 'studioWebPortal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#studio_web_portal_settings: {
                        let field_value = match fields_map.get("studioWebPortalSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'studioWebPortalSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tensor_board_app_settings: {
                        let field_value = match fields_map.get("tensorBoardAppSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'tensorBoardAppSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
