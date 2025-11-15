#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RemoteImageBuild {
    /// The configuration for the authentication
    #[builder(into)]
    #[serde(rename = "authConfigs")]
    pub r#auth_configs: Option<Vec<super::types::RemoteImageBuildAuthConfig>>,
    /// Set build-time variables
    #[builder(into)]
    #[serde(rename = "buildArg")]
    pub r#build_arg: Option<std::collections::HashMap<String, String>>,
    /// Pairs for build-time variables in the form TODO
    #[builder(into)]
    #[serde(rename = "buildArgs")]
    pub r#build_args: Option<std::collections::HashMap<String, String>>,
    /// BuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request.
    #[builder(into)]
    #[serde(rename = "buildId")]
    pub r#build_id: Option<String>,
    /// Images to consider as cache sources
    #[builder(into)]
    #[serde(rename = "cacheFroms")]
    pub r#cache_froms: Option<Vec<String>>,
    /// Optional parent cgroup for the container
    #[builder(into)]
    #[serde(rename = "cgroupParent")]
    pub r#cgroup_parent: Option<String>,
    /// Value to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. Please see https://docs.docker.com/build/building/context/ for more information about build contexts.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: String,
    /// The length of a CPU period in microseconds
    #[builder(into)]
    #[serde(rename = "cpuPeriod")]
    pub r#cpu_period: Option<i32>,
    /// Microseconds of CPU time that the container can get in a CPU period
    #[builder(into)]
    #[serde(rename = "cpuQuota")]
    pub r#cpu_quota: Option<i32>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0`, `1`)
    #[builder(into)]
    #[serde(rename = "cpuSetCpus")]
    pub r#cpu_set_cpus: Option<String>,
    /// MEMs in which to allow execution (`0-3`, `0`, `1`)
    #[builder(into)]
    #[serde(rename = "cpuSetMems")]
    pub r#cpu_set_mems: Option<String>,
    /// CPU shares (relative weight)
    #[builder(into)]
    #[serde(rename = "cpuShares")]
    pub r#cpu_shares: Option<i32>,
    /// Name of the Dockerfile. Defaults to `Dockerfile`.
    #[builder(into)]
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Option<String>,
    /// A list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form ["hostname:IP"]
    #[builder(into)]
    #[serde(rename = "extraHosts")]
    pub r#extra_hosts: Option<Vec<String>>,
    /// Always remove intermediate containers
    #[builder(into)]
    #[serde(rename = "forceRemove")]
    pub r#force_remove: Option<bool>,
    /// Isolation represents the isolation technology of a container. The supported values are
    #[builder(into)]
    #[serde(rename = "isolation")]
    pub r#isolation: Option<String>,
    /// Set metadata for an image
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<std::collections::HashMap<String, String>>,
    /// User-defined key/value metadata
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Set memory limit for build
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<i32>,
    /// Total memory (memory + swap), -1 to enable unlimited swap
    #[builder(into)]
    #[serde(rename = "memorySwap")]
    pub r#memory_swap: Option<i32>,
    /// Set the networking mode for the RUN instructions during build
    #[builder(into)]
    #[serde(rename = "networkMode")]
    pub r#network_mode: Option<String>,
    /// Do not use the cache when building the image
    #[builder(into)]
    #[serde(rename = "noCache")]
    pub r#no_cache: Option<bool>,
    /// Set platform if server is multi-platform capable
    #[builder(into)]
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
    /// Attempt to pull the image even if an older image exists locally
    #[builder(into)]
    #[serde(rename = "pullParent")]
    pub r#pull_parent: Option<bool>,
    /// A Git repository URI or HTTP/HTTPS context URI
    #[builder(into)]
    #[serde(rename = "remoteContext")]
    pub r#remote_context: Option<String>,
    /// Remove intermediate containers after a successful build. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "remove")]
    pub r#remove: Option<bool>,
    /// The security options
    #[builder(into)]
    #[serde(rename = "securityOpts")]
    pub r#security_opts: Option<Vec<String>>,
    /// Set an ID for the build session
    #[builder(into)]
    #[serde(rename = "sessionId")]
    pub r#session_id: Option<String>,
    /// Size of /dev/shm in bytes. The size must be greater than 0
    #[builder(into)]
    #[serde(rename = "shmSize")]
    pub r#shm_size: Option<i32>,
    /// If true the new layers are squashed into a new image with a single new layer
    #[builder(into)]
    #[serde(rename = "squash")]
    pub r#squash: Option<bool>,
    /// Suppress the build output and print image ID on success
    #[builder(into)]
    #[serde(rename = "suppressOutput")]
    pub r#suppress_output: Option<bool>,
    /// Name and optionally a tag in the 'name:tag' format
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// Set the target build stage to build
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    /// Configuration for ulimits
    #[builder(into)]
    #[serde(rename = "ulimits")]
    pub r#ulimits: Option<Vec<super::types::RemoteImageBuildUlimit>>,
    /// Version of the underlying builder to use
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
