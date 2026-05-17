#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudServiceConfigServerGitSettingRepository {
    /// A `http_basic_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpBasicAuth")]
    pub r#http_basic_auth: Option<Box<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingRepositoryHttpBasicAuth>>,
    /// The default label of the Git repository, should be the branch name, tag name, or commit-id of the repository.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// A name to identify on the Git repository, required only if repos exists.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// An array of strings used to match an application name. For each pattern, use the `{application}/{profile}` format with wildcards.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Option<Vec<String>>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Option<Vec<String>>,
    /// A `ssh_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Option<Box<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingRepositorySshAuth>>,
    /// The URI of the Git repository that's used as the Config Server back end should be started with `http://`, `https://`, `git@`, or `ssh://`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudServiceConfigServerGitSettingRepository {
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
                "http_basic_auth".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_basic_auth,
                )
                .await,
            );
            map.insert(
                "label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label,
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
                "patterns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#patterns,
                )
                .await,
            );
            map.insert(
                "search_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#search_paths,
                )
                .await,
            );
            map.insert(
                "ssh_auth".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_auth,
                )
                .await,
            );
            map.insert(
                "uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudServiceConfigServerGitSettingRepository {
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
                    r#http_basic_auth: {
                        let field_value = match fields_map.get("http_basic_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_basic_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ssh_auth: {
                        let field_value = match fields_map.get("ssh_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
