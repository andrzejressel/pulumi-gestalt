#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateSpecContainer {
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// (Optional, Deprecated)
    /// List of sources to populate environment variables in the container.
    /// All invalid keys will be reported as an event when the container is starting.
    /// When a key exists in multiple sources, the value associated with the last source will
    /// take precedence. Values defined by an Env with a duplicate key will take
    /// precedence.
    /// Structure is documented below.
    /// 
    /// > **Warning:** `env_from` is deprecated and will be removed in a future major release. This field is not supported by the Cloud Run API.
    #[builder(into)]
    #[serde(rename = "envFroms")]
    pub r#env_froms: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFrom>>,
    /// List of environment variables to set in the container.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerEnv>>,
    /// Docker image name. This is most often a reference to a container located
    /// in the container registry, such as gcr.io/cloudrun/hello
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. More info:
    /// https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "livenessProbe")]
    pub r#liveness_probe: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerLivenessProbe>>,
    /// Name of the container
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// List of open ports in the container.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerPort>>,
    /// Compute Resources required by this container. Used to set values such as max memory
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerResources>>,
    /// Startup probe of application within the container.
    /// All other probes are disabled if a startup probe is provided, until it
    /// succeeds. Container will not be added to service endpoints if the probe fails.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startupProbe")]
    pub r#startup_probe: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerStartupProbe>>,
    /// Volume to mount into the container's filesystem.
    /// Only supports SecretVolumeSources.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerVolumeMount>>,
    /// (Optional, Deprecated)
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    /// 
    /// > **Warning:** `working_dir` is deprecated and will be removed in a future major release. This field is not supported by the Cloud Run API.
    #[builder(into)]
    #[serde(rename = "workingDir")]
    pub r#working_dir: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplateSpecContainer {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

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
                "env_froms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#env_froms,
                )
                .await,
            );
            map.insert(
                "envs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#envs,
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
                "liveness_probe".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#liveness_probe,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ports,
                )
                .await,
            );
            map.insert(
                "resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resources,
                )
                .await,
            );
            map.insert(
                "startup_probe".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#startup_probe,
                )
                .await,
            );
            map.insert(
                "volume_mounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_mounts,
                )
                .await,
            );
            map.insert(
                "working_dir".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#working_dir,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplateSpecContainer {
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
                    r#env_froms: {
                        let field_value = match fields_map.get("env_froms") {
                            Some(value) => value,
                            None => bail!("Missing field 'env_froms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#envs: {
                        let field_value = match fields_map.get("envs") {
                            Some(value) => value,
                            None => bail!("Missing field 'envs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#liveness_probe: {
                        let field_value = match fields_map.get("liveness_probe") {
                            Some(value) => value,
                            None => bail!("Missing field 'liveness_probe' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ports: {
                        let field_value = match fields_map.get("ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#startup_probe: {
                        let field_value = match fields_map.get("startup_probe") {
                            Some(value) => value,
                            None => bail!("Missing field 'startup_probe' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_mounts: {
                        let field_value = match fields_map.get("volume_mounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_mounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#working_dir: {
                        let field_value = match fields_map.get("working_dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'working_dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
