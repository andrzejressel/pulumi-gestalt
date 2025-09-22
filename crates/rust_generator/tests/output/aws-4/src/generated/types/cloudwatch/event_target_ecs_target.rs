#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetEcsTarget {
    /// The capacity provider strategy to use for the task. If a `capacity_provider_strategy` specified, the `launch_type` parameter must be omitted. If no `capacity_provider_strategy` or `launch_type` is specified, the default capacity provider strategy for the cluster is used. Can be one or more. See below.
    #[builder(into)]
    #[serde(rename = "capacityProviderStrategies")]
    pub r#capacity_provider_strategies: Option<Vec<super::super::types::cloudwatch::EventTargetEcsTargetCapacityProviderStrategy>>,
    /// Specifies whether to enable Amazon ECS managed tags for the task.
    #[builder(into)]
    #[serde(rename = "enableEcsManagedTags")]
    pub r#enable_ecs_managed_tags: Option<bool>,
    /// Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task.
    #[builder(into)]
    #[serde(rename = "enableExecuteCommand")]
    pub r#enable_execute_command: Option<bool>,
    /// Specifies an ECS task group for the task. The maximum length is 255 characters.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: Option<String>,
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. Valid values include: `EC2`, `EXTERNAL`, or `FARGATE`.
    #[builder(into)]
    #[serde(rename = "launchType")]
    pub r#launch_type: Option<String>,
    /// Use this if the ECS task uses the awsvpc network mode. This specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. Required if `launch_type` is `FARGATE` because the awsvpc mode is required for Fargate tasks.
    #[builder(into)]
    #[serde(rename = "networkConfiguration")]
    pub r#network_configuration: Option<Box<super::super::types::cloudwatch::EventTargetEcsTargetNetworkConfiguration>>,
    /// An array of placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task.
    #[builder(into)]
    #[serde(rename = "orderedPlacementStrategies")]
    pub r#ordered_placement_strategies: Option<Vec<super::super::types::cloudwatch::EventTargetEcsTargetOrderedPlacementStrategy>>,
    /// An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime). See Below.
    #[builder(into)]
    #[serde(rename = "placementConstraints")]
    pub r#placement_constraints: Option<Vec<super::super::types::cloudwatch::EventTargetEcsTargetPlacementConstraint>>,
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as `1.1.0`. This is used only if LaunchType is FARGATE. For more information about valid platform versions, see [AWS Fargate Platform Versions](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html).
    #[builder(into)]
    #[serde(rename = "platformVersion")]
    pub r#platform_version: Option<String>,
    /// Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. The only valid value is: `TASK_DEFINITION`.
    #[builder(into)]
    #[serde(rename = "propagateTags")]
    pub r#propagate_tags: Option<String>,
    /// A map of tags to assign to ecs resources.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The number of tasks to create based on the TaskDefinition. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "taskCount")]
    pub r#task_count: Option<i32>,
    /// The ARN of the task definition to use if the event target is an Amazon ECS cluster.
    #[builder(into)]
    #[serde(rename = "taskDefinitionArn")]
    pub r#task_definition_arn: String,
}
