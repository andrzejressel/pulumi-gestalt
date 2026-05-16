#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppImageConfigJupyterLabImageConfigContainerConfig {
    /// The arguments for the container when you're running the application.
    #[builder(into)]
    #[serde(rename = "containerArguments")]
    pub r#container_arguments: Option<Vec<String>>,
    /// The entrypoint used to run the application in the container.
    #[builder(into)]
    #[serde(rename = "containerEntrypoints")]
    pub r#container_entrypoints: Option<Vec<String>>,
    /// The environment variables to set in the container.
    #[builder(into)]
    #[serde(rename = "containerEnvironmentVariables")]
    pub r#container_environment_variables: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppImageConfigJupyterLabImageConfigContainerConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("container_arguments".to_string(), self.r#container_arguments.to_pulumi_value().await);
            map.insert("container_entrypoints".to_string(), self.r#container_entrypoints.to_pulumi_value().await);
            map.insert("container_environment_variables".to_string(), self.r#container_environment_variables.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppImageConfigJupyterLabImageConfigContainerConfig {
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
                    r#container_arguments: {
                        let field_value = match fields_map.get("container_arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#container_entrypoints: {
                        let field_value = match fields_map.get("container_entrypoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_entrypoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#container_environment_variables: {
                        let field_value = match fields_map.get("container_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
