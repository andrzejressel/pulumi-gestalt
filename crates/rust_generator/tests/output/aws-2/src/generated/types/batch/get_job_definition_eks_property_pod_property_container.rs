#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionEksPropertyPodPropertyContainer {
    /// An array of arguments to the entrypoint
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Vec<String>,
    /// The command that's passed to the container.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Vec<String>,
    /// The environment variables to pass to a container.  Array of EksContainerEnvironmentVariable objects.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerEnv>,
    /// The image used to start a container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// The image pull policy for the container.
    #[builder(into)]
    #[serde(rename = "imagePullPolicy")]
    pub r#image_pull_policy: String,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type and amount of resources to assign to a container.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerResource>,
    /// The security context for a job.
    #[builder(into)]
    #[serde(rename = "securityContexts")]
    pub r#security_contexts: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerSecurityContext>,
    /// The volume mounts for the container.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerVolumeMount>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionEksPropertyPodPropertyContainer {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("args".to_string(), self.r#args.to_pulumi_value().await);
            map.insert("commands".to_string(), self.r#commands.to_pulumi_value().await);
            map.insert("envs".to_string(), self.r#envs.to_pulumi_value().await);
            map.insert("image".to_string(), self.r#image.to_pulumi_value().await);
            map.insert("image_pull_policy".to_string(), self.r#image_pull_policy.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("resources".to_string(), self.r#resources.to_pulumi_value().await);
            map.insert("security_contexts".to_string(), self.r#security_contexts.to_pulumi_value().await);
            map.insert("volume_mounts".to_string(), self.r#volume_mounts.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionEksPropertyPodPropertyContainer {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#args: {
                        let field_value = match fields_map.get("args") {
                            Some(value) => value,
                            None => bail!("Missing field 'args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#commands: {
                        let field_value = match fields_map.get("commands") {
                            Some(value) => value,
                            None => bail!("Missing field 'commands' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#envs: {
                        let field_value = match fields_map.get("envs") {
                            Some(value) => value,
                            None => bail!("Missing field 'envs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerEnv> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_pull_policy: {
                        let field_value = match fields_map.get("image_pull_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_pull_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerResource> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#security_contexts: {
                        let field_value = match fields_map.get("security_contexts") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_contexts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerSecurityContext> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#volume_mounts: {
                        let field_value = match fields_map.get("volume_mounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_mounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainerVolumeMount> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
