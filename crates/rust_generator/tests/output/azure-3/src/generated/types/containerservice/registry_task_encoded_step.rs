#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskEncodedStep {
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    #[serde(rename = "contextAccessToken")]
    pub r#context_access_token: Option<String>,
    /// The URL (absolute or relative) of the source context for this step.
    #[builder(into)]
    #[serde(rename = "contextPath")]
    pub r#context_path: Option<String>,
    /// Specifies a map of secret values that can be passed when running a task.
    #[builder(into)]
    #[serde(rename = "secretValues")]
    pub r#secret_values: Option<std::collections::HashMap<String, String>>,
    /// The (optionally base64 encoded) content of the build template.
    #[builder(into)]
    #[serde(rename = "taskContent")]
    pub r#task_content: String,
    /// The (optionally base64 encoded) content of the build parameters.
    #[builder(into)]
    #[serde(rename = "valueContent")]
    pub r#value_content: Option<String>,
    /// Specifies a map of values that can be passed when running a task.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistryTaskEncodedStep {
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
                "context_access_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#context_access_token,
                )
                .await,
            );
            map.insert(
                "context_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#context_path,
                )
                .await,
            );
            map.insert(
                "secret_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_values,
                )
                .await,
            );
            map.insert(
                "task_content".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_content,
                )
                .await,
            );
            map.insert(
                "value_content".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_content,
                )
                .await,
            );
            map.insert(
                "values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#values,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistryTaskEncodedStep {
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
                        let field_value = match fields_map.get("context_access_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'context_access_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#context_path: {
                        let field_value = match fields_map.get("context_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'context_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_values: {
                        let field_value = match fields_map.get("secret_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_content: {
                        let field_value = match fields_map.get("task_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_content: {
                        let field_value = match fields_map.get("value_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
