#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersEcsTaskParametersOverridesContainerOverride {
    /// List of commands to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// The number of cpu units reserved for the container, instead of the default value from the task definition. You must also specify a container name.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<i32>,
    /// A list of files containing the environment variables to pass to a container, instead of the value from the container definition. Detailed below.
    #[builder(into)]
    #[serde(rename = "environmentFiles")]
    pub r#environment_files: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile>>,
    /// The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name. Detailed below.
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment>>,
    /// The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<i32>,
    /// The soft limit (in MiB) of memory to reserve for the container, instead of the default value from the task definition. You must also specify a container name.
    #[builder(into)]
    #[serde(rename = "memoryReservation")]
    pub r#memory_reservation: Option<i32>,
    /// Name of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU. Detailed below.
    #[builder(into)]
    #[serde(rename = "resourceRequirements")]
    pub r#resource_requirements: Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement>>,
}
