#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParametersEcsTaskParameters {
    /// List of capacity provider strategies to use for the task. If a capacityProviderStrategy is specified, the launchType parameter must be omitted. If no capacityProviderStrategy or launchType is specified, the defaultCapacityProviderStrategy for the cluster is used. Detailed below.
    #[builder(into)]
    #[serde(rename = "capacityProviderStrategies")]
    pub r#capacity_provider_strategies: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersCapacityProviderStrategy>>,
    /// Specifies whether to enable Amazon ECS managed tags for the task. Valid values: true, false.
    #[builder(into)]
    #[serde(rename = "enableEcsManagedTags")]
    pub r#enable_ecs_managed_tags: Option<bool>,
    /// Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task. Valid values: true, false.
    #[builder(into)]
    #[serde(rename = "enableExecuteCommand")]
    pub r#enable_execute_command: Option<bool>,
    /// Specifies an Amazon ECS task group for the task. The maximum length is 255 characters.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: Option<String>,
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. The FARGATE value is supported only in the Regions where AWS Fargate with Amazon ECS is supported. Valid Values: EC2, FARGATE, EXTERNAL
    #[builder(into)]
    #[serde(rename = "launchType")]
    pub r#launch_type: Option<String>,
    /// Use this structure if the Amazon ECS task uses the awsvpc network mode. This structure specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. This structure is required if LaunchType is FARGATE because the awsvpc mode is required for Fargate tasks. If you specify NetworkConfiguration when the target ECS task does not use the awsvpc network mode, the task fails. Detailed below.
    #[builder(into)]
    #[serde(rename = "networkConfiguration")]
    pub r#network_configuration: Option<Box<super::super::types::pipes::PipeTargetParametersEcsTaskParametersNetworkConfiguration>>,
    /// The overrides that are associated with a task. Detailed below.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Option<Box<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverrides>>,
    /// An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime). Detailed below.
    #[builder(into)]
    #[serde(rename = "placementConstraints")]
    pub r#placement_constraints: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersPlacementConstraint>>,
    /// The placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task. Detailed below.
    #[builder(into)]
    #[serde(rename = "placementStrategies")]
    pub r#placement_strategies: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersPlacementStrategy>>,
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as 1.1.0. This structure is used only if LaunchType is FARGATE.
    #[builder(into)]
    #[serde(rename = "platformVersion")]
    pub r#platform_version: Option<String>,
    /// Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the TagResource API action. Valid Values: TASK_DEFINITION
    #[builder(into)]
    #[serde(rename = "propagateTags")]
    pub r#propagate_tags: Option<String>,
    /// The reference ID to use for the task. Maximum length of 1,024.
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Option<String>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The number of tasks to create based on TaskDefinition. The default is 1.
    #[builder(into)]
    #[serde(rename = "taskCount")]
    pub r#task_count: Option<i32>,
    /// The ARN of the task definition to use if the event target is an Amazon ECS task.
    #[builder(into)]
    #[serde(rename = "taskDefinitionArn")]
    pub r#task_definition_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeTargetParametersEcsTaskParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "capacity_provider_strategies",
                    &self.r#capacity_provider_strategies,
                ),
                to_pulumi_object_field(
                    "enable_ecs_managed_tags",
                    &self.r#enable_ecs_managed_tags,
                ),
                to_pulumi_object_field(
                    "enable_execute_command",
                    &self.r#enable_execute_command,
                ),
                to_pulumi_object_field(
                    "group",
                    &self.r#group,
                ),
                to_pulumi_object_field(
                    "launch_type",
                    &self.r#launch_type,
                ),
                to_pulumi_object_field(
                    "network_configuration",
                    &self.r#network_configuration,
                ),
                to_pulumi_object_field(
                    "overrides",
                    &self.r#overrides,
                ),
                to_pulumi_object_field(
                    "placement_constraints",
                    &self.r#placement_constraints,
                ),
                to_pulumi_object_field(
                    "placement_strategies",
                    &self.r#placement_strategies,
                ),
                to_pulumi_object_field(
                    "platform_version",
                    &self.r#platform_version,
                ),
                to_pulumi_object_field(
                    "propagate_tags",
                    &self.r#propagate_tags,
                ),
                to_pulumi_object_field(
                    "reference_id",
                    &self.r#reference_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "task_count",
                    &self.r#task_count,
                ),
                to_pulumi_object_field(
                    "task_definition_arn",
                    &self.r#task_definition_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeTargetParametersEcsTaskParameters {
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
                    r#capacity_provider_strategies: {
                        let field_value = match fields_map.get("capacity_provider_strategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_provider_strategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ecs_managed_tags: {
                        let field_value = match fields_map.get("enable_ecs_managed_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_ecs_managed_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_execute_command: {
                        let field_value = match fields_map.get("enable_execute_command") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_execute_command' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group: {
                        let field_value = match fields_map.get("group") {
                            Some(value) => value,
                            None => bail!("Missing field 'group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_type: {
                        let field_value = match fields_map.get("launch_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_configuration: {
                        let field_value = match fields_map.get("network_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overrides: {
                        let field_value = match fields_map.get("overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_constraints: {
                        let field_value = match fields_map.get("placement_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_strategies: {
                        let field_value = match fields_map.get("placement_strategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement_strategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform_version: {
                        let field_value = match fields_map.get("platform_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'platform_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#propagate_tags: {
                        let field_value = match fields_map.get("propagate_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'propagate_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_id: {
                        let field_value = match fields_map.get("reference_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_count: {
                        let field_value = match fields_map.get("task_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_definition_arn: {
                        let field_value = match fields_map.get("task_definition_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_definition_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
