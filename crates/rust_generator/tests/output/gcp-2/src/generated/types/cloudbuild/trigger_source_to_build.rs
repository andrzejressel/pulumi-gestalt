#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerSourceToBuild {
    /// The full resource name of the bitbucket server config.
    /// Format: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}.
    #[builder(into)]
    #[serde(rename = "bitbucketServerConfig")]
    pub r#bitbucket_server_config: Option<String>,
    /// The full resource name of the github enterprise config.
    /// Format: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}.
    #[builder(into)]
    #[serde(rename = "githubEnterpriseConfig")]
    pub r#github_enterprise_config: Option<String>,
    /// The branch or tag to use. Must start with "refs/" (required).
    #[builder(into)]
    #[serde(rename = "ref")]
    pub r#ref_: String,
    /// The type of the repo, since it may not be explicit from the repo field (e.g from a URL).
    /// Values can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER
    /// Possible values are: `UNKNOWN`, `CLOUD_SOURCE_REPOSITORIES`, `GITHUB`, `BITBUCKET_SERVER`.
    #[builder(into)]
    #[serde(rename = "repoType")]
    pub r#repo_type: String,
    /// The qualified resource name of the Repo API repository.
    /// Either uri or repository can be specified and is required.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Option<String>,
    /// The URI of the repo.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerSourceToBuild {
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
                    "bitbucket_server_config",
                    &self.r#bitbucket_server_config,
                ),
                to_pulumi_object_field(
                    "github_enterprise_config",
                    &self.r#github_enterprise_config,
                ),
                to_pulumi_object_field(
                    "ref_",
                    &self.r#ref_,
                ),
                to_pulumi_object_field(
                    "repo_type",
                    &self.r#repo_type,
                ),
                to_pulumi_object_field(
                    "repository",
                    &self.r#repository,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerSourceToBuild {
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
                    r#ref_: {
                        let field_value = match fields_map.get("ref_") {
                            Some(value) => value,
                            None => bail!("Missing field 'ref_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
