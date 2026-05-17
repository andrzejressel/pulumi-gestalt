#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskDockerStep {
    /// Specifies a map of arguments to be used when executing this step.
    #[builder(into)]
    #[serde(rename = "arguments")]
    pub r#arguments: Option<std::collections::HashMap<String, String>>,
    /// Should the image cache be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "cacheEnabled")]
    pub r#cache_enabled: Option<bool>,
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    #[serde(rename = "contextAccessToken")]
    pub r#context_access_token: String,
    /// The URL (absolute or relative) of the source context for this step. If the context is an url you can reference a specific branch or folder via `#branch:folder`.
    #[builder(into)]
    #[serde(rename = "contextPath")]
    pub r#context_path: String,
    /// The Dockerfile path relative to the source context.
    #[builder(into)]
    #[serde(rename = "dockerfilePath")]
    pub r#dockerfile_path: String,
    /// Specifies a list of fully qualified image names including the repository and tag.
    #[builder(into)]
    #[serde(rename = "imageNames")]
    pub r#image_names: Option<Vec<String>>,
    /// Should the image built be pushed to the registry or not? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "pushEnabled")]
    pub r#push_enabled: Option<bool>,
    /// Specifies a map of *secret* arguments to be used when executing this step.
    #[builder(into)]
    #[serde(rename = "secretArguments")]
    pub r#secret_arguments: Option<std::collections::HashMap<String, String>>,
    /// The name of the target build stage for the docker build.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistryTaskDockerStep {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "arguments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arguments,
                )
                .await,
            );
            map.insert(
                "cache_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_enabled,
                )
                .await,
            );
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
                "dockerfile_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dockerfile_path,
                )
                .await,
            );
            map.insert(
                "image_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_names,
                )
                .await,
            );
            map.insert(
                "push_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#push_enabled,
                )
                .await,
            );
            map.insert(
                "secret_arguments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_arguments,
                )
                .await,
            );
            map.insert(
                "target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistryTaskDockerStep {
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
                    r#arguments: {
                        let field_value = match fields_map.get("arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_enabled: {
                        let field_value = match fields_map.get("cache_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#dockerfile_path: {
                        let field_value = match fields_map.get("dockerfile_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerfile_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_names: {
                        let field_value = match fields_map.get("image_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#push_enabled: {
                        let field_value = match fields_map.get("push_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'push_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_arguments: {
                        let field_value = match fields_map.get("secret_arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
