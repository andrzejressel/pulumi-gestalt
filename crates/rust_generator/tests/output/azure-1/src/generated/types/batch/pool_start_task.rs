#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolStartTask {
    /// The command line executed by the start task.
    #[builder(into)]
    pub r#command_line: String,
    /// A map of strings (key,value) that represents the environment variables to set in the start task.
    #[builder(into)]
    pub r#common_environment_properties: Option<std::collections::HashMap<String, String>>,
    /// A `container` block is the settings for the container under which the start task runs as defined below. When this is specified, all directories recursively below the `AZ_BATCH_NODE_ROOT_DIR` (the root of Azure Batch directories on the node) are mapped into the container, all task environment variables are mapped into the container, and the task command line is executed in the container.
    #[builder(into)]
    pub r#containers: Option<Vec<super::super::types::batch::PoolStartTaskContainer>>,
    /// One or more `resource_file` blocks that describe the files to be downloaded to a compute node as defined below.
    #[builder(into)]
    pub r#resource_files: Option<Vec<super::super::types::batch::PoolStartTaskResourceFile>>,
    /// The number of retry count.
    #[builder(into)]
    pub r#task_retry_maximum: Option<i32>,
    /// A `user_identity` block that describes the user identity under which the start task runs as defined below.
    #[builder(into)]
    pub r#user_identity: Box<super::super::types::batch::PoolStartTaskUserIdentity>,
    /// A flag that indicates if the Batch pool should wait for the start task to be completed. Default to `false`.
    #[builder(into)]
    pub r#wait_for_success: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolStartTask {
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
                    "commandLine",
                    &self.r#command_line,
                ),
                to_pulumi_object_field(
                    "commonEnvironmentProperties",
                    &self.r#common_environment_properties,
                ),
                to_pulumi_object_field(
                    "containers",
                    &self.r#containers,
                ),
                to_pulumi_object_field(
                    "resourceFiles",
                    &self.r#resource_files,
                ),
                to_pulumi_object_field(
                    "taskRetryMaximum",
                    &self.r#task_retry_maximum,
                ),
                to_pulumi_object_field(
                    "userIdentity",
                    &self.r#user_identity,
                ),
                to_pulumi_object_field(
                    "waitForSuccess",
                    &self.r#wait_for_success,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("commandLine") {
                            Some(value) => value,
                            None => bail!("Missing field 'commandLine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_environment_properties: {
                        let field_value = match fields_map.get("commonEnvironmentProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'commonEnvironmentProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("resourceFiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceFiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_retry_maximum: {
                        let field_value = match fields_map.get("taskRetryMaximum") {
                            Some(value) => value,
                            None => bail!("Missing field 'taskRetryMaximum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_identity: {
                        let field_value = match fields_map.get("userIdentity") {
                            Some(value) => value,
                            None => bail!("Missing field 'userIdentity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wait_for_success: {
                        let field_value = match fields_map.get("waitForSuccess") {
                            Some(value) => value,
                            None => bail!("Missing field 'waitForSuccess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
