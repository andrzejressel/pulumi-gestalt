#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryRemoteRepositoryConfig {
    /// Specific settings for an Apt remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "aptRepository")]
    pub r#apt_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigAptRepository>>,
    /// Specific settings for an Artifact Registory remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "commonRepository")]
    pub r#common_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigCommonRepository>>,
    /// The description of the remote source.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// If true, the remote repository upstream and upstream credentials will
    /// not be validated.
    #[builder(into)]
    #[serde(rename = "disableUpstreamValidation")]
    pub r#disable_upstream_validation: Option<bool>,
    /// Specific settings for a Docker remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dockerRepository")]
    pub r#docker_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigDockerRepository>>,
    /// Specific settings for a Maven remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mavenRepository")]
    pub r#maven_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigMavenRepository>>,
    /// Specific settings for an Npm remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "npmRepository")]
    pub r#npm_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigNpmRepository>>,
    /// Specific settings for a Python remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pythonRepository")]
    pub r#python_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigPythonRepository>>,
    /// The credentials used to access the remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "upstreamCredentials")]
    pub r#upstream_credentials: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigUpstreamCredentials>>,
    /// Specific settings for an Yum remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "yumRepository")]
    pub r#yum_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigYumRepository>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryRemoteRepositoryConfig {
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
                    "apt_repository",
                    &self.r#apt_repository,
                ),
                to_pulumi_object_field(
                    "common_repository",
                    &self.r#common_repository,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "disable_upstream_validation",
                    &self.r#disable_upstream_validation,
                ),
                to_pulumi_object_field(
                    "docker_repository",
                    &self.r#docker_repository,
                ),
                to_pulumi_object_field(
                    "maven_repository",
                    &self.r#maven_repository,
                ),
                to_pulumi_object_field(
                    "npm_repository",
                    &self.r#npm_repository,
                ),
                to_pulumi_object_field(
                    "python_repository",
                    &self.r#python_repository,
                ),
                to_pulumi_object_field(
                    "upstream_credentials",
                    &self.r#upstream_credentials,
                ),
                to_pulumi_object_field(
                    "yum_repository",
                    &self.r#yum_repository,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryRemoteRepositoryConfig {
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
                    r#apt_repository: {
                        let field_value = match fields_map.get("apt_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'apt_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_repository: {
                        let field_value = match fields_map.get("common_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#docker_repository: {
                        let field_value = match fields_map.get("docker_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maven_repository: {
                        let field_value = match fields_map.get("maven_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'maven_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#npm_repository: {
                        let field_value = match fields_map.get("npm_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'npm_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_repository: {
                        let field_value = match fields_map.get("python_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#yum_repository: {
                        let field_value = match fields_map.get("yum_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'yum_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
