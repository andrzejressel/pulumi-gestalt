#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppImageConfigJupyterLabImageConfig {
    /// The configuration used to run the application image container. See Container Config details below.
    #[builder(into)]
    #[serde(rename = "containerConfig")]
    pub r#container_config: Option<Box<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfigContainerConfig>>,
    /// The URL where the Git repository is located. See File System Config details below.
    #[builder(into)]
    #[serde(rename = "fileSystemConfig")]
    pub r#file_system_config: Option<Box<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfigFileSystemConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppImageConfigJupyterLabImageConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("container_config".to_string(), self.r#container_config.to_pulumi_value().await);
            map.insert("file_system_config".to_string(), self.r#file_system_config.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppImageConfigJupyterLabImageConfig {
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
                    r#container_config: {
                        let field_value = match fields_map.get("container_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfigContainerConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#file_system_config: {
                        let field_value = match fields_map.get("file_system_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_system_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfigFileSystemConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
