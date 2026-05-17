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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobDefinitionEksPropertiesPodPropertiesContainers {
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
                "image_pull_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_pull_policy,
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
                "resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resources,
                )
                .await,
            );
            map.insert(
                "security_context".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_context,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobDefinitionEksPropertiesPodPropertiesContainers {
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
                    r#image_pull_policy: {
                        let field_value = match fields_map.get("image_pull_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_pull_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_context: {
                        let field_value = match fields_map.get("security_context") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
