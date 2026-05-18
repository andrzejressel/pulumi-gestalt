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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RemoteImageBuild {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auth_configs",
                    &self.r#auth_configs,
                ),
                to_pulumi_object_field(
                    "build_arg",
                    &self.r#build_arg,
                ),
                to_pulumi_object_field(
                    "build_args",
                    &self.r#build_args,
                ),
                to_pulumi_object_field(
                    "build_id",
                    &self.r#build_id,
                ),
                to_pulumi_object_field(
                    "cache_froms",
                    &self.r#cache_froms,
                ),
                to_pulumi_object_field(
                    "cgroup_parent",
                    &self.r#cgroup_parent,
                ),
                to_pulumi_object_field(
                    "context",
                    &self.r#context,
                ),
                to_pulumi_object_field(
                    "cpu_period",
                    &self.r#cpu_period,
                ),
                to_pulumi_object_field(
                    "cpu_quota",
                    &self.r#cpu_quota,
                ),
                to_pulumi_object_field(
                    "cpu_set_cpus",
                    &self.r#cpu_set_cpus,
                ),
                to_pulumi_object_field(
                    "cpu_set_mems",
                    &self.r#cpu_set_mems,
                ),
                to_pulumi_object_field(
                    "cpu_shares",
                    &self.r#cpu_shares,
                ),
                to_pulumi_object_field(
                    "dockerfile",
                    &self.r#dockerfile,
                ),
                to_pulumi_object_field(
                    "extra_hosts",
                    &self.r#extra_hosts,
                ),
                to_pulumi_object_field(
                    "force_remove",
                    &self.r#force_remove,
                ),
                to_pulumi_object_field(
                    "isolation",
                    &self.r#isolation,
                ),
                to_pulumi_object_field(
                    "label",
                    &self.r#label,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "memory",
                    &self.r#memory,
                ),
                to_pulumi_object_field(
                    "memory_swap",
                    &self.r#memory_swap,
                ),
                to_pulumi_object_field(
                    "network_mode",
                    &self.r#network_mode,
                ),
                to_pulumi_object_field(
                    "no_cache",
                    &self.r#no_cache,
                ),
                to_pulumi_object_field(
                    "platform",
                    &self.r#platform,
                ),
                to_pulumi_object_field(
                    "pull_parent",
                    &self.r#pull_parent,
                ),
                to_pulumi_object_field(
                    "remote_context",
                    &self.r#remote_context,
                ),
                to_pulumi_object_field(
                    "remove",
                    &self.r#remove,
                ),
                to_pulumi_object_field(
                    "security_opts",
                    &self.r#security_opts,
                ),
                to_pulumi_object_field(
                    "session_id",
                    &self.r#session_id,
                ),
                to_pulumi_object_field(
                    "shm_size",
                    &self.r#shm_size,
                ),
                to_pulumi_object_field(
                    "squash",
                    &self.r#squash,
                ),
                to_pulumi_object_field(
                    "suppress_output",
                    &self.r#suppress_output,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
                to_pulumi_object_field(
                    "ulimits",
                    &self.r#ulimits,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RemoteImageBuild {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#auth_configs: {
                        let field_value = match fields_map.get("auth_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#build_arg: {
                        let field_value = match fields_map.get("build_arg") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_arg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#build_args: {
                        let field_value = match fields_map.get("build_args") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#build_id: {
                        let field_value = match fields_map.get("build_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_froms: {
                        let field_value = match fields_map.get("cache_froms") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_froms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cgroup_parent: {
                        let field_value = match fields_map.get("cgroup_parent") {
                            Some(value) => value,
                            None => bail!("Missing field 'cgroup_parent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#context: {
                        let field_value = match fields_map.get("context") {
                            Some(value) => value,
                            None => bail!("Missing field 'context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_period: {
                        let field_value = match fields_map.get("cpu_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_quota: {
                        let field_value = match fields_map.get("cpu_quota") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_quota' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_set_cpus: {
                        let field_value = match fields_map.get("cpu_set_cpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_set_cpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_set_mems: {
                        let field_value = match fields_map.get("cpu_set_mems") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_set_mems' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_shares: {
                        let field_value = match fields_map.get("cpu_shares") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_shares' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dockerfile: {
                        let field_value = match fields_map.get("dockerfile") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerfile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extra_hosts: {
                        let field_value = match fields_map.get("extra_hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'extra_hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#force_remove: {
                        let field_value = match fields_map.get("force_remove") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_remove' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#isolation: {
                        let field_value = match fields_map.get("isolation") {
                            Some(value) => value,
                            None => bail!("Missing field 'isolation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory: {
                        let field_value = match fields_map.get("memory") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_swap: {
                        let field_value = match fields_map.get("memory_swap") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_swap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_mode: {
                        let field_value = match fields_map.get("network_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_cache: {
                        let field_value = match fields_map.get("no_cache") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_cache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform: {
                        let field_value = match fields_map.get("platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pull_parent: {
                        let field_value = match fields_map.get("pull_parent") {
                            Some(value) => value,
                            None => bail!("Missing field 'pull_parent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_context: {
                        let field_value = match fields_map.get("remote_context") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remove: {
                        let field_value = match fields_map.get("remove") {
                            Some(value) => value,
                            None => bail!("Missing field 'remove' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_opts: {
                        let field_value = match fields_map.get("security_opts") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_opts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_id: {
                        let field_value = match fields_map.get("session_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shm_size: {
                        let field_value = match fields_map.get("shm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'shm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#squash: {
                        let field_value = match fields_map.get("squash") {
                            Some(value) => value,
                            None => bail!("Missing field 'squash' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suppress_output: {
                        let field_value = match fields_map.get("suppress_output") {
                            Some(value) => value,
                            None => bail!("Missing field 'suppress_output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ulimits: {
                        let field_value = match fields_map.get("ulimits") {
                            Some(value) => value,
                            None => bail!("Missing field 'ulimits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
