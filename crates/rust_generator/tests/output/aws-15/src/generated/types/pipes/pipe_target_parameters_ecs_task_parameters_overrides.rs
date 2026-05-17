#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParametersEcsTaskParametersOverrides {
    /// One or more container overrides that are sent to a task. Detailed below.
    #[builder(into)]
    #[serde(rename = "containerOverrides")]
    pub r#container_overrides: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesContainerOverride>>,
    /// The number of cpu units reserved for the container, instead of the default value from the task definition. You must also specify a container name.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<String>,
    /// The ephemeral storage setting override for the task.  Detailed below.
    #[builder(into)]
    #[serde(rename = "ephemeralStorage")]
    pub r#ephemeral_storage: Option<Box<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage>>,
    /// The Amazon Resource Name (ARN) of the task execution IAM role override for the task.
    #[builder(into)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Option<String>,
    /// List of Elastic Inference accelerator overrides for the task. Detailed below.
    #[builder(into)]
    #[serde(rename = "inferenceAcceleratorOverrides")]
    pub r#inference_accelerator_overrides: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride>>,
    /// The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<String>,
    /// The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.
    #[builder(into)]
    #[serde(rename = "taskRoleArn")]
    pub r#task_role_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeTargetParametersEcsTaskParametersOverrides {
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
                "container_overrides".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_overrides,
                )
                .await,
            );
            map.insert(
                "cpu".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu,
                )
                .await,
            );
            map.insert(
                "ephemeral_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ephemeral_storage,
                )
                .await,
            );
            map.insert(
                "execution_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_role_arn,
                )
                .await,
            );
            map.insert(
                "inference_accelerator_overrides".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inference_accelerator_overrides,
                )
                .await,
            );
            map.insert(
                "memory".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory,
                )
                .await,
            );
            map.insert(
                "task_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_role_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeTargetParametersEcsTaskParametersOverrides {
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
                    r#container_overrides: {
                        let field_value = match fields_map.get("container_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu: {
                        let field_value = match fields_map.get("cpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_storage: {
                        let field_value = match fields_map.get("ephemeral_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeral_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role_arn: {
                        let field_value = match fields_map.get("execution_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_accelerator_overrides: {
                        let field_value = match fields_map.get("inference_accelerator_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_accelerator_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory: {
                        let field_value = match fields_map.get("memory") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_role_arn: {
                        let field_value = match fields_map.get("task_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
