#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTemplateSpecContainer {
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Vec<String>,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Vec<String>,
    /// List of sources to populate environment variables in the container.
    /// All invalid keys will be reported as an event when the container is starting.
    /// When a key exists in multiple sources, the value associated with the last source will
    /// take precedence. Values defined by an Env with a duplicate key will take
    /// precedence.
    #[builder(into)]
    #[serde(rename = "envFroms")]
    pub r#env_froms: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvFrom>,
    /// List of environment variables to set in the container.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnv>,
    /// Docker image name. This is most often a reference to a container located
    /// in the container registry, such as gcr.io/cloudrun/hello
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails.
    #[builder(into)]
    #[serde(rename = "livenessProbes")]
    pub r#liveness_probes: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerLivenessProbe>,
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// List of open ports in the container.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerPort>,
    /// Compute Resources required by this container. Used to set values such as max memory
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerResource>,
    /// Startup probe of application within the container.
    /// All other probes are disabled if a startup probe is provided, until it
    /// succeeds. Container will not be added to service endpoints if the probe fails.
    #[builder(into)]
    #[serde(rename = "startupProbes")]
    pub r#startup_probes: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerStartupProbe>,
    /// Volume to mount into the container's filesystem.
    /// Only supports SecretVolumeSources.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerVolumeMount>,
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    #[builder(into)]
    #[serde(rename = "workingDir")]
    pub r#working_dir: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceTemplateSpecContainer {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "args",
                    &self.r#args,
                ),
                to_pulumi_object_field(
                    "commands",
                    &self.r#commands,
                ),
                to_pulumi_object_field(
                    "env_froms",
                    &self.r#env_froms,
                ),
                to_pulumi_object_field(
                    "envs",
                    &self.r#envs,
                ),
                to_pulumi_object_field(
                    "image",
                    &self.r#image,
                ),
                to_pulumi_object_field(
                    "liveness_probes",
                    &self.r#liveness_probes,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "ports",
                    &self.r#ports,
                ),
                to_pulumi_object_field(
                    "resources",
                    &self.r#resources,
                ),
                to_pulumi_object_field(
                    "startup_probes",
                    &self.r#startup_probes,
                ),
                to_pulumi_object_field(
                    "volume_mounts",
                    &self.r#volume_mounts,
                ),
                to_pulumi_object_field(
                    "working_dir",
                    &self.r#working_dir,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceTemplateSpecContainer {
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
                    r#liveness_probes: {
                        let field_value = match fields_map.get("liveness_probes") {
                            Some(value) => value,
                            None => bail!("Missing field 'liveness_probes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#startup_probes: {
                        let field_value = match fields_map.get("startup_probes") {
                            Some(value) => value,
                            None => bail!("Missing field 'startup_probes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
