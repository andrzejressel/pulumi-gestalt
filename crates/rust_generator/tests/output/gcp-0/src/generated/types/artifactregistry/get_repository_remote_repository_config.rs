#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRepositoryRemoteRepositoryConfig {
    /// Specific settings for an Apt remote repository.
    #[builder(into)]
    #[serde(rename = "aptRepositories")]
    pub r#apt_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigAptRepository>,
    /// Specific settings for an Artifact Registory remote repository.
    #[builder(into)]
    #[serde(rename = "commonRepositories")]
    pub r#common_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigCommonRepository>,
    /// The description of the remote source.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// If true, the remote repository upstream and upstream credentials will
    /// not be validated.
    #[builder(into)]
    #[serde(rename = "disableUpstreamValidation")]
    pub r#disable_upstream_validation: bool,
    /// Specific settings for a Docker remote repository.
    #[builder(into)]
    #[serde(rename = "dockerRepositories")]
    pub r#docker_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigDockerRepository>,
    /// Specific settings for a Maven remote repository.
    #[builder(into)]
    #[serde(rename = "mavenRepositories")]
    pub r#maven_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigMavenRepository>,
    /// Specific settings for an Npm remote repository.
    #[builder(into)]
    #[serde(rename = "npmRepositories")]
    pub r#npm_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigNpmRepository>,
    /// Specific settings for a Python remote repository.
    #[builder(into)]
    #[serde(rename = "pythonRepositories")]
    pub r#python_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigPythonRepository>,
    /// The credentials used to access the remote repository.
    #[builder(into)]
    #[serde(rename = "upstreamCredentials")]
    pub r#upstream_credentials: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigUpstreamCredential>,
    /// Specific settings for an Yum remote repository.
    #[builder(into)]
    #[serde(rename = "yumRepositories")]
    pub r#yum_repositories: Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigYumRepository>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRepositoryRemoteRepositoryConfig {
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
                    "aptRepositories",
                    &self.r#apt_repositories,
                ),
                to_pulumi_object_field(
                    "commonRepositories",
                    &self.r#common_repositories,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "disableUpstreamValidation",
                    &self.r#disable_upstream_validation,
                ),
                to_pulumi_object_field(
                    "dockerRepositories",
                    &self.r#docker_repositories,
                ),
                to_pulumi_object_field(
                    "mavenRepositories",
                    &self.r#maven_repositories,
                ),
                to_pulumi_object_field(
                    "npmRepositories",
                    &self.r#npm_repositories,
                ),
                to_pulumi_object_field(
                    "pythonRepositories",
                    &self.r#python_repositories,
                ),
                to_pulumi_object_field(
                    "upstreamCredentials",
                    &self.r#upstream_credentials,
                ),
                to_pulumi_object_field(
                    "yumRepositories",
                    &self.r#yum_repositories,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRepositoryRemoteRepositoryConfig {
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
                    r#apt_repositories: {
                        let field_value = match fields_map.get("aptRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'aptRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_repositories: {
                        let field_value = match fields_map.get("commonRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'commonRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_upstream_validation: {
                        let field_value = match fields_map.get("disableUpstreamValidation") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableUpstreamValidation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_repositories: {
                        let field_value = match fields_map.get("dockerRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maven_repositories: {
                        let field_value = match fields_map.get("mavenRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'mavenRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#npm_repositories: {
                        let field_value = match fields_map.get("npmRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'npmRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_repositories: {
                        let field_value = match fields_map.get("pythonRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'pythonRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upstream_credentials: {
                        let field_value = match fields_map.get("upstreamCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'upstreamCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#yum_repositories: {
                        let field_value = match fields_map.get("yumRepositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'yumRepositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
