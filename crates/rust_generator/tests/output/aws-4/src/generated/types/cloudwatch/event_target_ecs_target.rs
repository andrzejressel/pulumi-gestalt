#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventTargetEcsTarget {
    /// The capacity provider strategy to use for the task. If a `capacity_provider_strategy` specified, the `launch_type` parameter must be omitted. If no `capacity_provider_strategy` or `launch_type` is specified, the default capacity provider strategy for the cluster is used. Can be one or more. See below.
    #[builder(into)]
    pub r#capacity_provider_strategies: Option<Vec<super::super::types::cloudwatch::EventTargetEcsTargetCapacityProviderStrategy>>,
    /// Specifies whether to enable Amazon ECS managed tags for the task.
    #[builder(into)]
    pub r#enable_ecs_managed_tags: Option<bool>,
    /// Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task.
    #[builder(into)]
    pub r#enable_execute_command: Option<bool>,
    /// Specifies an ECS task group for the task. The maximum length is 255 characters.
    #[builder(into)]
    pub r#group: Option<String>,
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. Valid values include: `EC2`, `EXTERNAL`, or `FARGATE`.
    #[builder(into)]
    pub r#launch_type: Option<String>,
    /// Use this if the ECS task uses the awsvpc network mode. This specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. Required if `launch_type` is `FARGATE` because the awsvpc mode is required for Fargate tasks.
    #[builder(into)]
    pub r#network_configuration: Option<Box<super::super::types::cloudwatch::EventTargetEcsTargetNetworkConfiguration>>,
    /// An array of placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task.
    #[builder(into)]
    pub r#ordered_placement_strategies: Option<Vec<super::super::types::cloudwatch::EventTargetEcsTargetOrderedPlacementStrategy>>,
    /// An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime). See Below.
    #[builder(into)]
    pub r#placement_constraints: Option<Vec<super::super::types::cloudwatch::EventTargetEcsTargetPlacementConstraint>>,
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as `1.1.0`. This is used only if LaunchType is FARGATE. For more information about valid platform versions, see [AWS Fargate Platform Versions](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html).
    #[builder(into)]
    pub r#platform_version: Option<String>,
    /// Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. The only valid value is: `TASK_DEFINITION`.
    #[builder(into)]
    pub r#propagate_tags: Option<String>,
    /// A map of tags to assign to ecs resources.
    #[builder(into)]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The number of tasks to create based on the TaskDefinition. Defaults to `1`.
    #[builder(into)]
    pub r#task_count: Option<i32>,
    /// The ARN of the task definition to use if the event target is an Amazon ECS cluster.
    #[builder(into)]
    pub r#task_definition_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventTargetEcsTarget {
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
                    "orderedPlacementStrategies",
                    &self.r#ordered_placement_strategies,
                ),
                to_pulumi_object_field(
                    "placementConstraints",
                    &self.r#placement_constraints,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventTargetEcsTarget {
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
                    r#ordered_placement_strategies: {
                        let field_value = match fields_map.get("orderedPlacementStrategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'orderedPlacementStrategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
