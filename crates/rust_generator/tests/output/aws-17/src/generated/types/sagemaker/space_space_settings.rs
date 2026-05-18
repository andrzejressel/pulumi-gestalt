#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpaceSpaceSettings {
    /// The type of app created within the space.
    #[builder(into)]
    #[serde(rename = "appType")]
    pub r#app_type: Option<String>,
    /// The Code Editor application settings. See `code_editor_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "codeEditorAppSettings")]
    pub r#code_editor_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsCodeEditorAppSettings>>,
    /// A file system, created by you, that you assign to a space for an Amazon SageMaker Domain. See `custom_file_system` Block below.
    #[builder(into)]
    #[serde(rename = "customFileSystems")]
    pub r#custom_file_systems: Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsCustomFileSystem>>,
    /// The settings for the JupyterLab application. See `jupyter_lab_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterLabAppSettings")]
    pub r#jupyter_lab_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See `jupyter_server_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterServerAppSettings")]
    pub r#jupyter_server_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See `kernel_gateway_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "kernelGatewayAppSettings")]
    pub r#kernel_gateway_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsKernelGatewayAppSettings>>,
    /// The storage settings. See `space_storage_settings` Block below.
    #[builder(into)]
    #[serde(rename = "spaceStorageSettings")]
    pub r#space_storage_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsSpaceStorageSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpaceSpaceSettings {
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
                    "app_type",
                    &self.r#app_type,
                ),
                to_pulumi_object_field(
                    "code_editor_app_settings",
                    &self.r#code_editor_app_settings,
                ),
                to_pulumi_object_field(
                    "custom_file_systems",
                    &self.r#custom_file_systems,
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
                    "space_storage_settings",
                    &self.r#space_storage_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpaceSpaceSettings {
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
                    r#app_type: {
                        let field_value = match fields_map.get("app_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#custom_file_systems: {
                        let field_value = match fields_map.get("custom_file_systems") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_file_systems' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
