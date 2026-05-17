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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "apt_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apt_repositories,
                )
                .await,
            );
            map.insert(
                "common_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#common_repositories,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "disable_upstream_validation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_upstream_validation,
                )
                .await,
            );
            map.insert(
                "docker_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#docker_repositories,
                )
                .await,
            );
            map.insert(
                "maven_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maven_repositories,
                )
                .await,
            );
            map.insert(
                "npm_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#npm_repositories,
                )
                .await,
            );
            map.insert(
                "python_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#python_repositories,
                )
                .await,
            );
            map.insert(
                "upstream_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upstream_credentials,
                )
                .await,
            );
            map.insert(
                "yum_repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#yum_repositories,
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
                        let field_value = match fields_map.get("apt_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'apt_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_repositories: {
                        let field_value = match fields_map.get("common_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("disable_upstream_validation") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_upstream_validation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_repositories: {
                        let field_value = match fields_map.get("docker_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maven_repositories: {
                        let field_value = match fields_map.get("maven_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'maven_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#npm_repositories: {
                        let field_value = match fields_map.get("npm_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'npm_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_repositories: {
                        let field_value = match fields_map.get("python_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upstream_credentials: {
                        let field_value = match fields_map.get("upstream_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'upstream_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#yum_repositories: {
                        let field_value = match fields_map.get("yum_repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'yum_repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
