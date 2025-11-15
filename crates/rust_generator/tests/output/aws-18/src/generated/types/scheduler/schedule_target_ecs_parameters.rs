#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduleTargetEcsParameters {
    /// Up to `6` capacity provider strategies to use for the task. Detailed below.
    #[builder(into)]
    #[serde(rename = "capacityProviderStrategies")]
    pub r#capacity_provider_strategies: Option<Vec<super::super::types::scheduler::ScheduleTargetEcsParametersCapacityProviderStrategy>>,
    /// Specifies whether to enable Amazon ECS managed tags for the task. For more information, see [Tagging Your Amazon ECS Resources](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html) in the Amazon ECS Developer Guide.
    #[builder(into)]
    #[serde(rename = "enableEcsManagedTags")]
    pub r#enable_ecs_managed_tags: Option<bool>,
    /// Specifies whether to enable the execute command functionality for the containers in this task.
    #[builder(into)]
    #[serde(rename = "enableExecuteCommand")]
    pub r#enable_execute_command: Option<bool>,
    /// Specifies an ECS task group for the task. At most 255 characters.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: Option<String>,
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. One of: `EC2`, `FARGATE`, `EXTERNAL`.
    #[builder(into)]
    #[serde(rename = "launchType")]
    pub r#launch_type: Option<String>,
    /// Configures the networking associated with the task. Detailed below.
    #[builder(into)]
    #[serde(rename = "networkConfiguration")]
    pub r#network_configuration: Option<Box<super::super::types::scheduler::ScheduleTargetEcsParametersNetworkConfiguration>>,
    /// A set of up to 10 placement constraints to use for the task. Detailed below.
    #[builder(into)]
    #[serde(rename = "placementConstraints")]
    pub r#placement_constraints: Option<Vec<super::super::types::scheduler::ScheduleTargetEcsParametersPlacementConstraint>>,
    /// A set of up to 5 placement strategies. Detailed below.
    #[builder(into)]
    #[serde(rename = "placementStrategies")]
    pub r#placement_strategies: Option<Vec<super::super::types::scheduler::ScheduleTargetEcsParametersPlacementStrategy>>,
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as `1.1.0`.
    #[builder(into)]
    #[serde(rename = "platformVersion")]
    pub r#platform_version: Option<String>,
    /// Specifies whether to propagate the tags from the task definition to the task. One of: `TASK_DEFINITION`.
    #[builder(into)]
    #[serde(rename = "propagateTags")]
    pub r#propagate_tags: Option<String>,
    /// Reference ID to use for the task.
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Option<String>,
    /// The metadata that you apply to the task. Each tag consists of a key and an optional value. For more information, see [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) in the Amazon ECS API Reference.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The number of tasks to create. Ranges from `1` (default) to `10`.
    #[builder(into)]
    #[serde(rename = "taskCount")]
    pub r#task_count: Option<i32>,
    /// ARN of the task definition to use.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "taskDefinitionArn")]
    pub r#task_definition_arn: String,
}
