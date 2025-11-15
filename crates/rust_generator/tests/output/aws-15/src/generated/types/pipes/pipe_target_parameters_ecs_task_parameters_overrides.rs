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
