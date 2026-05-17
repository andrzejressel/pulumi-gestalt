#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfigurationCodeRepository {
    /// Configuration for building and running the service from a source code repository. See Code Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "codeConfiguration")]
    pub r#code_configuration: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepositoryCodeConfiguration>>,
    /// Location of the repository that contains the source code.
    #[builder(into)]
    #[serde(rename = "repositoryUrl")]
    pub r#repository_url: String,
    /// Version that should be used within the source code repository. See Source Code Version below for more details.
    #[builder(into)]
    #[serde(rename = "sourceCodeVersion")]
    pub r#source_code_version: Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepositorySourceCodeVersion>,
    /// The path of the directory that stores source code and configuration files. The build and start commands also execute from here. The path is absolute from root and, if not specified, defaults to the repository root.
    #[builder(into)]
    #[serde(rename = "sourceDirectory")]
    pub r#source_directory: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfigurationCodeRepository {
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
                    "code_configuration",
                    &self.r#code_configuration,
                ),
                to_pulumi_object_field(
                    "repository_url",
                    &self.r#repository_url,
                ),
                to_pulumi_object_field(
                    "source_code_version",
                    &self.r#source_code_version,
                ),
                to_pulumi_object_field(
                    "source_directory",
                    &self.r#source_directory,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfigurationCodeRepository {
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
                    r#code_configuration: {
                        let field_value = match fields_map.get("code_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository_url: {
                        let field_value = match fields_map.get("repository_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_code_version: {
                        let field_value = match fields_map.get("source_code_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_code_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_directory: {
                        let field_value = match fields_map.get("source_directory") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_directory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
