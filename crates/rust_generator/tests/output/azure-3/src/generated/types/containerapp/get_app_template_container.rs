#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAppTemplateContainer {
    /// A list of extra arguments to pass to the container.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Vec<String>,
    /// A command to pass to the container to override the default. This is provided as a list of command line elements without spaces.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Vec<String>,
    /// The amount of vCPU to allocate to the container. Possible values include `0.25`, `0.5`, `0.75`, `1.0`, `1.25`, `1.5`, `1.75`, and `2.0`.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: f64,
    /// One or more `env` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Vec<super::super::types::containerapp::GetAppTemplateContainerEnv>,
    /// The amount of ephemeral storage available to the Container App.
    #[builder(into)]
    #[serde(rename = "ephemeralStorage")]
    pub r#ephemeral_storage: String,
    /// The image to use to create the container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// A `liveness_probe` block as detailed below.
    #[builder(into)]
    #[serde(rename = "livenessProbes")]
    pub r#liveness_probes: Vec<super::super::types::containerapp::GetAppTemplateContainerLivenessProbe>,
    /// The amount of memory to allocate to the container. Possible values include `0.5Gi`, `1Gi`, `1.5Gi`, `2Gi`, `2.5Gi`, `3Gi`, `3.5Gi`, and `4Gi`.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: String,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `readiness_probe` block as detailed below.
    #[builder(into)]
    #[serde(rename = "readinessProbes")]
    pub r#readiness_probes: Vec<super::super::types::containerapp::GetAppTemplateContainerReadinessProbe>,
    /// A `startup_probe` block as detailed below.
    #[builder(into)]
    #[serde(rename = "startupProbes")]
    pub r#startup_probes: Vec<super::super::types::containerapp::GetAppTemplateContainerStartupProbe>,
    /// A `volume_mounts` block as detailed below.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Vec<super::super::types::containerapp::GetAppTemplateContainerVolumeMount>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAppTemplateContainer {
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
                "cpu".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu,
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
                "ephemeral_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ephemeral_storage,
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
                "liveness_probes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#liveness_probes,
                )
                .await,
            );
            map.insert(
                "memory".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory,
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
                "readiness_probes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#readiness_probes,
                )
                .await,
            );
            map.insert(
                "startup_probes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#startup_probes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAppTemplateContainer {
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
                    r#cpu: {
                        let field_value = match fields_map.get("cpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ephemeral_storage: {
                        let field_value = match fields_map.get("ephemeral_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeral_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#memory: {
                        let field_value = match fields_map.get("memory") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#readiness_probes: {
                        let field_value = match fields_map.get("readiness_probes") {
                            Some(value) => value,
                            None => bail!("Missing field 'readiness_probes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
