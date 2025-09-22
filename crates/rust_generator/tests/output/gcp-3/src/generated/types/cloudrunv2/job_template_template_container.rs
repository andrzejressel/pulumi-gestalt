#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateTemplateContainer {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references are not supported in Cloud Run.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// List of environment variables to set in the container.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Option<Vec<super::super::types::cloudrunv2::JobTemplateTemplateContainerEnv>>,
    /// URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Name of the container specified as a DNS_LABEL.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// List of ports to expose from the container. Only a single port can be specified. The specified ports must be listening on all interfaces (0.0.0.0) within the container to be accessible.
    /// If omitted, a port number will be chosen and passed to the container through the PORT environment variable for the container to listen on
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<super::super::types::cloudrunv2::JobTemplateTemplateContainerPort>>,
    /// Compute Resource requirements by this container. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<super::super::types::cloudrunv2::JobTemplateTemplateContainerResources>>,
    /// Volume to mount into the container's filesystem.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Option<Vec<super::super::types::cloudrunv2::JobTemplateTemplateContainerVolumeMount>>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image.
    #[builder(into)]
    #[serde(rename = "workingDir")]
    pub r#working_dir: Option<String>,
}
