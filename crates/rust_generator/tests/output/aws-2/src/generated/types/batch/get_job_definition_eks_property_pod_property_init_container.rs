#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionEksPropertyPodPropertyInitContainer {
    /// An array of arguments to the entrypoint
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Vec<String>,
    /// The command that's passed to the container.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Vec<String>,
    /// The environment variables to pass to a container.  Array of EksContainerEnvironmentVariable objects.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyInitContainerEnv>,
    /// The image used to start a container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// The image pull policy for the container.
    #[builder(into)]
    #[serde(rename = "imagePullPolicy")]
    pub r#image_pull_policy: String,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type and amount of resources to assign to a container.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyInitContainerResource>,
    /// The security context for a job.
    #[builder(into)]
    #[serde(rename = "securityContexts")]
    pub r#security_contexts: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyInitContainerSecurityContext>,
    /// The volume mounts for the container.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyInitContainerVolumeMount>,
}
