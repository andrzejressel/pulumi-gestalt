#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTaskExecutionOverrides {
    /// One or more container overrides that are sent to a task. See below.
    #[builder(into)]
    #[serde(rename = "containerOverrides")]
    pub r#container_overrides: Option<Vec<super::super::types::ecs::GetTaskExecutionOverridesContainerOverride>>,
    /// The CPU override for the task.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<String>,
    /// Amazon Resource Name (ARN) of the task execution role override for the task.
    #[builder(into)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Option<String>,
    /// Elastic Inference accelerator override for the task. See below.
    #[builder(into)]
    #[serde(rename = "inferenceAcceleratorOverrides")]
    pub r#inference_accelerator_overrides: Option<Vec<super::super::types::ecs::GetTaskExecutionOverridesInferenceAcceleratorOverride>>,
    /// The memory override for the task.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<String>,
    /// Amazon Resource Name (ARN) of the role that containers in this task can assume.
    #[builder(into)]
    #[serde(rename = "taskRoleArn")]
    pub r#task_role_arn: Option<String>,
}
