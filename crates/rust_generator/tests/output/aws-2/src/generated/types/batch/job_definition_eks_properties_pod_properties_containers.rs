#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobDefinitionEksPropertiesPodPropertiesContainers {
    /// Array of arguments to the entrypoint. If this isn't specified, the CMD of the container image is used. This corresponds to the args member in the Entrypoint portion of the Pod in Kubernetes. Environment variable references are expanded using the container's environment.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// Entrypoint for the container. This isn't run within a shell. If this isn't specified, the ENTRYPOINT of the container image is used. Environment variable references are expanded using the container's environment.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// Environment variables to pass to a container. See EKS Environment below.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesContainersEnv>>,
    /// Docker image used to start the container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Image pull policy for the container. Supported values are `Always`, `IfNotPresent`, and `Never`.
    #[builder(into)]
    #[serde(rename = "imagePullPolicy")]
    pub r#image_pull_policy: Option<String>,
    /// Name of the container. If the name isn't specified, the default name "Default" is used. Each container in a pod must have a unique name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Type and amount of resources to assign to a container. The supported resources include `memory`, `cpu`, and `nvidia.com/gpu`.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Box<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesContainersResources>>,
    /// Security context for a job.
    #[builder(into)]
    #[serde(rename = "securityContext")]
    pub r#security_context: Option<Box<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesContainersSecurityContext>>,
    /// Volume mounts for the container.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesContainersVolumeMount>>,
}
