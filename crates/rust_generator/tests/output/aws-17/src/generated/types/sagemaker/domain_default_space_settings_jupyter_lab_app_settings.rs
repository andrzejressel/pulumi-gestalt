#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultSpaceSettingsJupyterLabAppSettings {
    /// Indicates whether idle shutdown is activated for JupyterLab applications. see `app_lifecycle_management` Block below.
    #[builder(into)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement>>,
    /// The lifecycle configuration that runs before the default lifecycle configuration. It can override changes made in the default lifecycle configuration.
    #[builder(into)]
    #[serde(rename = "builtInLifecycleConfigArn")]
    pub r#built_in_lifecycle_config_arn: Option<String>,
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterServer application. see `code_repository` Block below.
    #[builder(into)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsCodeRepository>>,
    /// A list of custom SageMaker images that are configured to run as a JupyterLab app. see `custom_image` Block below.
    #[builder(into)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsCustomImage>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec>>,
    /// The configuration parameters that specify the IAM roles assumed by the execution role of SageMaker (assumable roles) and the cluster instances or job execution environments (execution roles or runtime roles) to manage and access resources required for running Amazon EMR clusters or Amazon EMR Serverless applications. see `emr_settings` Block below.
    #[builder(into)]
    #[serde(rename = "emrSettings")]
    pub r#emr_settings: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsEmrSettings>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultSpaceSettingsJupyterLabAppSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("app_lifecycle_management".to_string(), self.r#app_lifecycle_management.to_pulumi_value().await);
            map.insert("built_in_lifecycle_config_arn".to_string(), self.r#built_in_lifecycle_config_arn.to_pulumi_value().await);
            map.insert("code_repositories".to_string(), self.r#code_repositories.to_pulumi_value().await);
            map.insert("custom_images".to_string(), self.r#custom_images.to_pulumi_value().await);
            map.insert("default_resource_spec".to_string(), self.r#default_resource_spec.to_pulumi_value().await);
            map.insert("emr_settings".to_string(), self.r#emr_settings.to_pulumi_value().await);
            map.insert("lifecycle_config_arns".to_string(), self.r#lifecycle_config_arns.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultSpaceSettingsJupyterLabAppSettings {
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
                    r#app_lifecycle_management: {
                        let field_value = match fields_map.get("app_lifecycle_management") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_lifecycle_management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#built_in_lifecycle_config_arn: {
                        let field_value = match fields_map.get("built_in_lifecycle_config_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'built_in_lifecycle_config_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#code_repositories: {
                        let field_value = match fields_map.get("code_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsCodeRepository>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_images: {
                        let field_value = match fields_map.get("custom_images") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_images' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsCustomImage>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_resource_spec: {
                        let field_value = match fields_map.get("default_resource_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_resource_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#emr_settings: {
                        let field_value = match fields_map.get("emr_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'emr_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsEmrSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_config_arns: {
                        let field_value = match fields_map.get("lifecycle_config_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_config_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
