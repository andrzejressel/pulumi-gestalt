#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskFileStep {
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    pub r#context_access_token: Option<String>,
    /// The URL (absolute or relative) of the source context for this step.
    #[builder(into)]
    pub r#context_path: Option<String>,
    /// Specifies a map of secret values that can be passed when running a task.
    #[builder(into)]
    pub r#secret_values: Option<std::collections::BTreeMap<String, String>>,
    /// The task template file path relative to the source context.
    #[builder(into)]
    pub r#task_file_path: String,
    /// The parameters file path relative to the source context.
    #[builder(into)]
    pub r#value_file_path: Option<String>,
    /// Specifies a map of values that can be passed when running a task.
    #[builder(into)]
    pub r#values: Option<std::collections::BTreeMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistryTaskFileStep {
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
                    "contextAccessToken",
                    &self.r#context_access_token,
                ),
                to_pulumi_object_field(
                    "contextPath",
                    &self.r#context_path,
                ),
                to_pulumi_object_field(
                    "secretValues",
                    &self.r#secret_values,
                ),
                to_pulumi_object_field(
                    "taskFilePath",
                    &self.r#task_file_path,
                ),
                to_pulumi_object_field(
                    "valueFilePath",
                    &self.r#value_file_path,
                ),
                to_pulumi_object_field(
                    "values",
                    &self.r#values,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistryTaskFileStep {
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
                    r#context_access_token: {
                        let field_value = match fields_map.get("contextAccessToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'contextAccessToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#context_path: {
                        let field_value = match fields_map.get("contextPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'contextPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_values: {
                        let field_value = match fields_map.get("secretValues") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretValues' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_file_path: {
                        let field_value = match fields_map.get("taskFilePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'taskFilePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_file_path: {
                        let field_value = match fields_map.get("valueFilePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'valueFilePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#values: {
                        let field_value = match fields_map.get("values") {
                            Some(value) => value,
                            None => bail!("Missing field 'values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
