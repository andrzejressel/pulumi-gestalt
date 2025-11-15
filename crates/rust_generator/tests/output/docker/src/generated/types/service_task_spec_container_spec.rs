#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecContainerSpec {
    /// Arguments to the command
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// The command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// References to zero or more configs that will be exposed to the service
    #[builder(into)]
    #[serde(rename = "configs")]
    pub r#configs: Option<Vec<super::types::ServiceTaskSpecContainerSpecConfig>>,
    /// The working directory for commands to run in
    #[builder(into)]
    #[serde(rename = "dir")]
    pub r#dir: Option<String>,
    /// Specification for DNS related configurations in resolver configuration file (`resolv.conf`)
    #[builder(into)]
    #[serde(rename = "dnsConfig")]
    pub r#dns_config: Option<Box<super::types::ServiceTaskSpecContainerSpecDnsConfig>>,
    /// A list of environment variables in the form VAR="value"
    #[builder(into)]
    #[serde(rename = "env")]
    pub r#env: Option<std::collections::HashMap<String, String>>,
    /// A list of additional groups that the container process will run as
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    /// A test to perform to check that the container is healthy
    #[builder(into)]
    #[serde(rename = "healthcheck")]
    pub r#healthcheck: Option<Box<super::types::ServiceTaskSpecContainerSpecHealthcheck>>,
    /// The hostname to use for the container, as a valid RFC 1123 hostname
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    /// A list of hostname/IP mappings to add to the container's hosts file
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Option<Vec<super::types::ServiceTaskSpecContainerSpecHost>>,
    /// The image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker.RemoteImage` with the `repo_digest` or `docker.RegistryImage` with the `name` attribute for this, as shown in the examples.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Isolation technology of the containers running the service. (Windows only). Defaults to `default`.
    #[builder(into)]
    #[serde(rename = "isolation")]
    pub r#isolation: Option<String>,
    /// User-defined key/value metadata
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<super::types::ServiceTaskSpecContainerSpecLabel>>,
    /// Specification for mounts to be added to containers created as part of the service
    #[builder(into)]
    #[serde(rename = "mounts")]
    pub r#mounts: Option<Vec<super::types::ServiceTaskSpecContainerSpecMount>>,
    /// Security options for the container
    #[builder(into)]
    #[serde(rename = "privileges")]
    pub r#privileges: Option<Box<super::types::ServiceTaskSpecContainerSpecPrivileges>>,
    /// Mount the container's root filesystem as read only
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// References to zero or more secrets that will be exposed to the service
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Option<Vec<super::types::ServiceTaskSpecContainerSpecSecret>>,
    /// Amount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate.
    #[builder(into)]
    #[serde(rename = "stopGracePeriod")]
    pub r#stop_grace_period: Option<String>,
    /// Signal to stop the container
    #[builder(into)]
    #[serde(rename = "stopSignal")]
    pub r#stop_signal: Option<String>,
    /// Sysctls config (Linux only)
    #[builder(into)]
    #[serde(rename = "sysctl")]
    pub r#sysctl: Option<std::collections::HashMap<String, String>>,
    /// The user inside the container
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}
