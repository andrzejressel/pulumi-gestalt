#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpaceSpaceSettingsJupyterLabAppSettings {
    /// Settings that are used to configure and manage the lifecycle of JupyterLab applications in a space. See `app_lifecycle_management` Block below.
    #[builder(into)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement>>,
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterLab application. See `code_repository` Block below.
    #[builder(into)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsCodeRepository>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. See `default_resource_spec` Block below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpaceSpaceSettingsJupyterLabAppSettings {
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
            map.insert("code_repositories".to_string(), self.r#code_repositories.to_pulumi_value().await);
            map.insert("default_resource_spec".to_string(), self.r#default_resource_spec.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpaceSpaceSettingsJupyterLabAppSettings {
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
                        <Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#code_repositories: {
                        let field_value = match fields_map.get("code_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsCodeRepository>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_resource_spec: {
                        let field_value = match fields_map.get("default_resource_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_resource_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
