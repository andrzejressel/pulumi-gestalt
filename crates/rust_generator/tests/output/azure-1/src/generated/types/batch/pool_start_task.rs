#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolStartTask {
    /// The command line executed by the start task.
    #[builder(into)]
    #[serde(rename = "commandLine")]
    pub r#command_line: String,
    /// A map of strings (key,value) that represents the environment variables to set in the start task.
    #[builder(into)]
    #[serde(rename = "commonEnvironmentProperties")]
    pub r#common_environment_properties: Option<std::collections::HashMap<String, String>>,
    /// A `container` block is the settings for the container under which the start task runs as defined below. When this is specified, all directories recursively below the `AZ_BATCH_NODE_ROOT_DIR` (the root of Azure Batch directories on the node) are mapped into the container, all task environment variables are mapped into the container, and the task command line is executed in the container.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Option<Vec<super::super::types::batch::PoolStartTaskContainer>>,
    /// One or more `resource_file` blocks that describe the files to be downloaded to a compute node as defined below.
    #[builder(into)]
    #[serde(rename = "resourceFiles")]
    pub r#resource_files: Option<Vec<super::super::types::batch::PoolStartTaskResourceFile>>,
    /// The number of retry count.
    #[builder(into)]
    #[serde(rename = "taskRetryMaximum")]
    pub r#task_retry_maximum: Option<i32>,
    /// A `user_identity` block that describes the user identity under which the start task runs as defined below.
    #[builder(into)]
    #[serde(rename = "userIdentity")]
    pub r#user_identity: Box<super::super::types::batch::PoolStartTaskUserIdentity>,
    /// A flag that indicates if the Batch pool should wait for the start task to be completed. Default to `false`.
    #[builder(into)]
    #[serde(rename = "waitForSuccess")]
    pub r#wait_for_success: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolStartTask {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "command_line",
                    &self.r#command_line,
                ),
                to_pulumi_object_field(
                    "common_environment_properties",
                    &self.r#common_environment_properties,
                ),
                to_pulumi_object_field(
                    "containers",
                    &self.r#containers,
                ),
                to_pulumi_object_field(
                    "resource_files",
                    &self.r#resource_files,
                ),
                to_pulumi_object_field(
                    "task_retry_maximum",
                    &self.r#task_retry_maximum,
                ),
                to_pulumi_object_field(
                    "user_identity",
                    &self.r#user_identity,
                ),
                to_pulumi_object_field(
                    "wait_for_success",
                    &self.r#wait_for_success,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PoolStartTask {
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
                    r#command_line: {
                        let field_value = match fields_map.get("command_line") {
                            Some(value) => value,
                            None => bail!("Missing field 'command_line' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_environment_properties: {
                        let field_value = match fields_map.get("common_environment_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_environment_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#containers: {
                        let field_value = match fields_map.get("containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_files: {
                        let field_value = match fields_map.get("resource_files") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_retry_maximum: {
                        let field_value = match fields_map.get("task_retry_maximum") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_retry_maximum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_identity: {
                        let field_value = match fields_map.get("user_identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wait_for_success: {
                        let field_value = match fields_map.get("wait_for_success") {
                            Some(value) => value,
                            None => bail!("Missing field 'wait_for_success' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
