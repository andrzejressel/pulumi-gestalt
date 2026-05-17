#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSpringCloudServiceConfigServerGitSettingRepository {
    /// A `http_basic_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpBasicAuths")]
    pub r#http_basic_auths: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingRepositoryHttpBasicAuth>,
    /// The default label of the Git repository, which is a branch name, tag name, or commit-id of the repository
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: String,
    /// Specifies The name of the Spring Cloud Service resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// An array of strings used to match an application name. For each pattern, use the `{application}/{profile}` format with wildcards.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Vec<String>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Vec<String>,
    /// A `ssh_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshAuths")]
    pub r#ssh_auths: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingRepositorySshAuth>,
    /// The URI of the Git repository
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSpringCloudServiceConfigServerGitSettingRepository {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "http_basic_auths",
                    &self.r#http_basic_auths,
                ),
                to_pulumi_object_field(
                    "label",
                    &self.r#label,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "patterns",
                    &self.r#patterns,
                ),
                to_pulumi_object_field(
                    "search_paths",
                    &self.r#search_paths,
                ),
                to_pulumi_object_field(
                    "ssh_auths",
                    &self.r#ssh_auths,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSpringCloudServiceConfigServerGitSettingRepository {
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
                    r#http_basic_auths: {
                        let field_value = match fields_map.get("http_basic_auths") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_basic_auths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#patterns: {
                        let field_value = match fields_map.get("patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#search_paths: {
                        let field_value = match fields_map.get("search_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'search_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_auths: {
                        let field_value = match fields_map.get("ssh_auths") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_auths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
