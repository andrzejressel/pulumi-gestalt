#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduleTargetEcsParameters {
    /// Up to `6` capacity provider strategies to use for the task. Detailed below.
    #[builder(into)]
    pub r#capacity_provider_strategies: Option<Vec<super::super::types::scheduler::ScheduleTargetEcsParametersCapacityProviderStrategy>>,
    /// Specifies whether to enable Amazon ECS managed tags for the task. For more information, see [Tagging Your Amazon ECS Resources](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html) in the Amazon ECS Developer Guide.
    #[builder(into)]
    pub r#enable_ecs_managed_tags: Option<bool>,
    /// Specifies whether to enable the execute command functionality for the containers in this task.
    #[builder(into)]
    pub r#enable_execute_command: Option<bool>,
    /// Specifies an ECS task group for the task. At most 255 characters.
    #[builder(into)]
    pub r#group: Option<String>,
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. One of: `EC2`, `FARGATE`, `EXTERNAL`.
    #[builder(into)]
    pub r#launch_type: Option<String>,
    /// Configures the networking associated with the task. Detailed below.
    #[builder(into)]
    pub r#network_configuration: Option<Box<super::super::types::scheduler::ScheduleTargetEcsParametersNetworkConfiguration>>,
    /// A set of up to 10 placement constraints to use for the task. Detailed below.
    #[builder(into)]
    pub r#placement_constraints: Option<Vec<super::super::types::scheduler::ScheduleTargetEcsParametersPlacementConstraint>>,
    /// A set of up to 5 placement strategies. Detailed below.
    #[builder(into)]
    pub r#placement_strategies: Option<Vec<super::super::types::scheduler::ScheduleTargetEcsParametersPlacementStrategy>>,
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as `1.1.0`.
    #[builder(into)]
    pub r#platform_version: Option<String>,
    /// Specifies whether to propagate the tags from the task definition to the task. One of: `TASK_DEFINITION`.
    #[builder(into)]
    pub r#propagate_tags: Option<String>,
    /// Reference ID to use for the task.
    #[builder(into)]
    pub r#reference_id: Option<String>,
    /// The metadata that you apply to the task. Each tag consists of a key and an optional value. For more information, see [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) in the Amazon ECS API Reference.
    #[builder(into)]
    pub r#tags: Option<std::collections::BTreeMap<String, String>>,
    /// The number of tasks to create. Ranges from `1` (default) to `10`.
    #[builder(into)]
    pub r#task_count: Option<i32>,
    /// ARN of the task definition to use.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    pub r#task_definition_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduleTargetEcsParameters {
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
                    "capacityProviderStrategies",
                    &self.r#capacity_provider_strategies,
                ),
                to_pulumi_object_field(
                    "enableEcsManagedTags",
                    &self.r#enable_ecs_managed_tags,
                ),
                to_pulumi_object_field(
                    "enableExecuteCommand",
                    &self.r#enable_execute_command,
                ),
                to_pulumi_object_field(
                    "group",
                    &self.r#group,
                ),
                to_pulumi_object_field(
                    "launchType",
                    &self.r#launch_type,
                ),
                to_pulumi_object_field(
                    "networkConfiguration",
                    &self.r#network_configuration,
                ),
                to_pulumi_object_field(
                    "placementConstraints",
                    &self.r#placement_constraints,
                ),
                to_pulumi_object_field(
                    "placementStrategies",
                    &self.r#placement_strategies,
                ),
                to_pulumi_object_field(
                    "platformVersion",
                    &self.r#platform_version,
                ),
                to_pulumi_object_field(
                    "propagateTags",
                    &self.r#propagate_tags,
                ),
                to_pulumi_object_field(
                    "referenceId",
                    &self.r#reference_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "taskCount",
                    &self.r#task_count,
                ),
                to_pulumi_object_field(
                    "taskDefinitionArn",
                    &self.r#task_definition_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduleTargetEcsParameters {
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
                        let field_value = match fields_map.get("capacityProviderStrategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacityProviderStrategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ecs_managed_tags: {
                        let field_value = match fields_map.get("enableEcsManagedTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableEcsManagedTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_execute_command: {
                        let field_value = match fields_map.get("enableExecuteCommand") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableExecuteCommand' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("launchType") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_configuration: {
                        let field_value = match fields_map.get("networkConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_constraints: {
                        let field_value = match fields_map.get("placementConstraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'placementConstraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_strategies: {
                        let field_value = match fields_map.get("placementStrategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'placementStrategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform_version: {
                        let field_value = match fields_map.get("platformVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'platformVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#propagate_tags: {
                        let field_value = match fields_map.get("propagateTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'propagateTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_id: {
                        let field_value = match fields_map.get("referenceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'referenceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("taskCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'taskCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_definition_arn: {
                        let field_value = match fields_map.get("taskDefinitionArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'taskDefinitionArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
