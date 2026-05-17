#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerGitFileSource {
    /// The full resource name of the bitbucket server config.
    /// Format: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}.
    #[builder(into)]
    #[serde(rename = "bitbucketServerConfig")]
    pub r#bitbucket_server_config: String,
    /// The full resource name of the github enterprise config.
    /// Format: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}.
    #[builder(into)]
    #[serde(rename = "githubEnterpriseConfig")]
    pub r#github_enterprise_config: String,
    /// The path of the file, with the repo root as the root of the path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// The type of the repo, since it may not be explicit from the repo field (e.g from a URL).
    /// Values can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: ["UNKNOWN", "CLOUD_SOURCE_REPOSITORIES", "GITHUB", "BITBUCKET_SERVER"]
    #[builder(into)]
    #[serde(rename = "repoType")]
    pub r#repo_type: String,
    /// The fully qualified resource name of the Repo API repository. The fully qualified resource name of the Repo API repository.
    /// If unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: String,
    /// The branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the
    /// filename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions
    /// If unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: String,
    /// The URI of the repo (optional). If unspecified, the repo from which the trigger
    /// invocation originated is assumed to be the repo from which to read the specified path.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerGitFileSource {
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
                "bitbucket_server_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bitbucket_server_config,
                )
                .await,
            );
            map.insert(
                "github_enterprise_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#github_enterprise_config,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "repo_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repo_type,
                )
                .await,
            );
            map.insert(
                "repository".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repository,
                )
                .await,
            );
            map.insert(
                "revision".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#revision,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerGitFileSource {
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
                    r#bitbucket_server_config: {
                        let field_value = match fields_map.get("bitbucket_server_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitbucket_server_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#github_enterprise_config: {
                        let field_value = match fields_map.get("github_enterprise_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'github_enterprise_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repo_type: {
                        let field_value = match fields_map.get("repo_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'repo_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository: {
                        let field_value = match fields_map.get("repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revision: {
                        let field_value = match fields_map.get("revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
