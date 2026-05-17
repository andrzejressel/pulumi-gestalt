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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTaskSpecContainerSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "args".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#args,
                )
                .await,
            );
            map.insert(
                "commands".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#commands,
                )
                .await,
            );
            map.insert(
                "configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configs,
                )
                .await,
            );
            map.insert(
                "dir".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dir,
                )
                .await,
            );
            map.insert(
                "dns_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_config,
                )
                .await,
            );
            map.insert(
                "env".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#env,
                )
                .await,
            );
            map.insert(
                "groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#groups,
                )
                .await,
            );
            map.insert(
                "healthcheck".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#healthcheck,
                )
                .await,
            );
            map.insert(
                "hostname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hostname,
                )
                .await,
            );
            map.insert(
                "hosts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hosts,
                )
                .await,
            );
            map.insert(
                "image".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image,
                )
                .await,
            );
            map.insert(
                "isolation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#isolation,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "mounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mounts,
                )
                .await,
            );
            map.insert(
                "privileges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#privileges,
                )
                .await,
            );
            map.insert(
                "read_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#read_only,
                )
                .await,
            );
            map.insert(
                "secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets,
                )
                .await,
            );
            map.insert(
                "stop_grace_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stop_grace_period,
                )
                .await,
            );
            map.insert(
                "stop_signal".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stop_signal,
                )
                .await,
            );
            map.insert(
                "sysctl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sysctl,
                )
                .await,
            );
            map.insert(
                "user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTaskSpecContainerSpec {
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
                    r#args: {
                        let field_value = match fields_map.get("args") {
                            Some(value) => value,
                            None => bail!("Missing field 'args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#commands: {
                        let field_value = match fields_map.get("commands") {
                            Some(value) => value,
                            None => bail!("Missing field 'commands' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configs: {
                        let field_value = match fields_map.get("configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dir: {
                        let field_value = match fields_map.get("dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_config: {
                        let field_value = match fields_map.get("dns_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#env: {
                        let field_value = match fields_map.get("env") {
                            Some(value) => value,
                            None => bail!("Missing field 'env' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#groups: {
                        let field_value = match fields_map.get("groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#healthcheck: {
                        let field_value = match fields_map.get("healthcheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthcheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hosts: {
                        let field_value = match fields_map.get("hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mounts: {
                        let field_value = match fields_map.get("mounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'mounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privileges: {
                        let field_value = match fields_map.get("privileges") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only: {
                        let field_value = match fields_map.get("read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stop_grace_period: {
                        let field_value = match fields_map.get("stop_grace_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'stop_grace_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stop_signal: {
                        let field_value = match fields_map.get("stop_signal") {
                            Some(value) => value,
                            None => bail!("Missing field 'stop_signal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sysctl: {
                        let field_value = match fields_map.get("sysctl") {
                            Some(value) => value,
                            None => bail!("Missing field 'sysctl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user: {
                        let field_value = match fields_map.get("user") {
                            Some(value) => value,
                            None => bail!("Missing field 'user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
