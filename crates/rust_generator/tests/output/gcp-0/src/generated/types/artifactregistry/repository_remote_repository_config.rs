#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryRemoteRepositoryConfig {
    /// Specific settings for an Apt remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#apt_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigAptRepository>>,
    /// Specific settings for an Artifact Registory remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#common_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigCommonRepository>>,
    /// The description of the remote source.
    #[builder(into)]
    pub r#description: Option<String>,
    /// If true, the remote repository upstream and upstream credentials will
    /// not be validated.
    #[builder(into)]
    pub r#disable_upstream_validation: Option<bool>,
    /// Specific settings for a Docker remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#docker_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigDockerRepository>>,
    /// Specific settings for a Maven remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#maven_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigMavenRepository>>,
    /// Specific settings for an Npm remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#npm_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigNpmRepository>>,
    /// Specific settings for a Python remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#python_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigPythonRepository>>,
    /// The credentials used to access the remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#upstream_credentials: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigUpstreamCredentials>>,
    /// Specific settings for an Yum remote repository.
    /// Structure is documented below.
    #[builder(into)]
    pub r#yum_repository: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigYumRepository>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryRemoteRepositoryConfig {
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
                    "aptRepository",
                    &self.r#apt_repository,
                ),
                to_pulumi_object_field(
                    "commonRepository",
                    &self.r#common_repository,
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
                    "dockerRepository",
                    &self.r#docker_repository,
                ),
                to_pulumi_object_field(
                    "mavenRepository",
                    &self.r#maven_repository,
                ),
                to_pulumi_object_field(
                    "npmRepository",
                    &self.r#npm_repository,
                ),
                to_pulumi_object_field(
                    "pythonRepository",
                    &self.r#python_repository,
                ),
                to_pulumi_object_field(
                    "upstreamCredentials",
                    &self.r#upstream_credentials,
                ),
                to_pulumi_object_field(
                    "yumRepository",
                    &self.r#yum_repository,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("aptRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'aptRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_repository: {
                        let field_value = match fields_map.get("commonRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'commonRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#docker_repository: {
                        let field_value = match fields_map.get("dockerRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maven_repository: {
                        let field_value = match fields_map.get("mavenRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'mavenRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#npm_repository: {
                        let field_value = match fields_map.get("npmRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'npmRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_repository: {
                        let field_value = match fields_map.get("pythonRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'pythonRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#yum_repository: {
                        let field_value = match fields_map.get("yumRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'yumRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
