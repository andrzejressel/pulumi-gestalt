#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupContainer {
    /// A list of commands which should be run on the container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// The required number of CPU cores of the containers. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: f64,
    /// The upper limit of the number of CPU cores of the containers.
    #[builder(into)]
    #[serde(rename = "cpuLimit")]
    pub r#cpu_limit: Option<f64>,
    /// A list of environment variables to be set on the container. Specified as a map of name/value pairs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// The container image name. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// The definition of a readiness probe for this container as documented in the `liveness_probe` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "livenessProbe")]
    pub r#liveness_probe: Option<Box<super::super::types::containerservice::GroupContainerLivenessProbe>>,
    /// The required memory of the containers in GB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: f64,
    /// The upper limit of the memory of the containers in GB.
    #[builder(into)]
    #[serde(rename = "memoryLimit")]
    pub r#memory_limit: Option<f64>,
    /// Specifies the name of the Container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A set of public ports for the container. Changing this forces a new resource to be created. Set as documented in the `ports` block below.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<super::super::types::containerservice::GroupContainerPort>>,
    /// The definition of a readiness probe for this container as documented in the `readiness_probe` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "readinessProbe")]
    pub r#readiness_probe: Option<Box<super::super::types::containerservice::GroupContainerReadinessProbe>>,
    /// A list of sensitive environment variables to be set on the container. Specified as a map of name/value pairs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secureEnvironmentVariables")]
    pub r#secure_environment_variables: Option<std::collections::HashMap<String, String>>,
    /// The definition of the security context for this container as documented in the `security` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "securities")]
    pub r#securities: Option<Vec<super::super::types::containerservice::GroupContainerSecurity>>,
    /// The definition of a volume mount for this container as documented in the `volume` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::containerservice::GroupContainerVolume>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupContainer {
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
                    "commands",
                    &self.r#commands,
                ),
                to_pulumi_object_field(
                    "cpu",
                    &self.r#cpu,
                ),
                to_pulumi_object_field(
                    "cpu_limit",
                    &self.r#cpu_limit,
                ),
                to_pulumi_object_field(
                    "environment_variables",
                    &self.r#environment_variables,
                ),
                to_pulumi_object_field(
                    "image",
                    &self.r#image,
                ),
                to_pulumi_object_field(
                    "liveness_probe",
                    &self.r#liveness_probe,
                ),
                to_pulumi_object_field(
                    "memory",
                    &self.r#memory,
                ),
                to_pulumi_object_field(
                    "memory_limit",
                    &self.r#memory_limit,
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
                    "readiness_probe",
                    &self.r#readiness_probe,
                ),
                to_pulumi_object_field(
                    "secure_environment_variables",
                    &self.r#secure_environment_variables,
                ),
                to_pulumi_object_field(
                    "securities",
                    &self.r#securities,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupContainer {
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
                    r#cpu_limit: {
                        let field_value = match fields_map.get("cpu_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#memory: {
                        let field_value = match fields_map.get("memory") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_limit: {
                        let field_value = match fields_map.get("memory_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#readiness_probe: {
                        let field_value = match fields_map.get("readiness_probe") {
                            Some(value) => value,
                            None => bail!("Missing field 'readiness_probe' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secure_environment_variables: {
                        let field_value = match fields_map.get("secure_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'secure_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#securities: {
                        let field_value = match fields_map.get("securities") {
                            Some(value) => value,
                            None => bail!("Missing field 'securities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volumes: {
                        let field_value = match fields_map.get("volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
