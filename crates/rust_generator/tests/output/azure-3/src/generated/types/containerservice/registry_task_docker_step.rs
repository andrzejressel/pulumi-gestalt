#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskDockerStep {
    /// Specifies a map of arguments to be used when executing this step.
    #[builder(into)]
    pub r#arguments: Option<std::collections::BTreeMap<String, String>>,
    /// Should the image cache be enabled? Defaults to `true`.
    #[builder(into)]
    pub r#cache_enabled: Option<bool>,
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    pub r#context_access_token: String,
    /// The URL (absolute or relative) of the source context for this step. If the context is an url you can reference a specific branch or folder via `#branch:folder`.
    #[builder(into)]
    pub r#context_path: String,
    /// The Dockerfile path relative to the source context.
    #[builder(into)]
    pub r#dockerfile_path: String,
    /// Specifies a list of fully qualified image names including the repository and tag.
    #[builder(into)]
    pub r#image_names: Option<Vec<String>>,
    /// Should the image built be pushed to the registry or not? Defaults to `true`.
    #[builder(into)]
    pub r#push_enabled: Option<bool>,
    /// Specifies a map of *secret* arguments to be used when executing this step.
    #[builder(into)]
    pub r#secret_arguments: Option<std::collections::BTreeMap<String, String>>,
    /// The name of the target build stage for the docker build.
    #[builder(into)]
    pub r#target: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistryTaskDockerStep {
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
                    "arguments",
                    &self.r#arguments,
                ),
                to_pulumi_object_field(
                    "cacheEnabled",
                    &self.r#cache_enabled,
                ),
                to_pulumi_object_field(
                    "contextAccessToken",
                    &self.r#context_access_token,
                ),
                to_pulumi_object_field(
                    "contextPath",
                    &self.r#context_path,
                ),
                to_pulumi_object_field(
                    "dockerfilePath",
                    &self.r#dockerfile_path,
                ),
                to_pulumi_object_field(
                    "imageNames",
                    &self.r#image_names,
                ),
                to_pulumi_object_field(
                    "pushEnabled",
                    &self.r#push_enabled,
                ),
                to_pulumi_object_field(
                    "secretArguments",
                    &self.r#secret_arguments,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("cacheEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#dockerfile_path: {
                        let field_value = match fields_map.get("dockerfilePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerfilePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_names: {
                        let field_value = match fields_map.get("imageNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#push_enabled: {
                        let field_value = match fields_map.get("pushEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'pushEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_arguments: {
                        let field_value = match fields_map.get("secretArguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretArguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
