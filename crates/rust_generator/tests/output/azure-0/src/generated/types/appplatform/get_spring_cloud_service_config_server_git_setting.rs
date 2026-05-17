#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSpringCloudServiceConfigServerGitSetting {
    /// A `http_basic_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpBasicAuths")]
    pub r#http_basic_auths: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingHttpBasicAuth>,
    /// The default label of the Git repository, which is a branch name, tag name, or commit-id of the repository
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: String,
    /// One or more `repository` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "repositories")]
    pub r#repositories: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingRepository>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Vec<String>,
    /// A `ssh_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshAuths")]
    pub r#ssh_auths: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingSshAuth>,
    /// The URI of the Git repository
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSpringCloudServiceConfigServerGitSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "http_basic_auths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_basic_auths,
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
                "repositories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repositories,
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
                "ssh_auths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_auths,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSpringCloudServiceConfigServerGitSetting {
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
                    r#repositories: {
                        let field_value = match fields_map.get("repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
