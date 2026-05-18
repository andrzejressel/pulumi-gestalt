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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "container_arguments",
                    &self.r#container_arguments,
                ),
                to_pulumi_object_field(
                    "container_entrypoints",
                    &self.r#container_entrypoints,
                ),
                to_pulumi_object_field(
                    "container_environment_variables",
                    &self.r#container_environment_variables,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppImageConfigJupyterLabImageConfigContainerConfig {
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
                    r#container_arguments: {
                        let field_value = match fields_map.get("container_arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_entrypoints: {
                        let field_value = match fields_map.get("container_entrypoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_entrypoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_environment_variables: {
                        let field_value = match fields_map.get("container_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
