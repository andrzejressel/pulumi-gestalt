#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceGithubRepo {
    /// Specifies the GitHub account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// Specifies the collaboration branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// Specifies the GitHub Enterprise host name. For example: <https://github.mydomain.com>.
    /// 
    /// > **Note:** You must log in to the Synapse UI to complete the authentication to the GitHub repository.
    #[builder(into)]
    #[serde(rename = "gitUrl")]
    pub r#git_url: Option<String>,
    /// The last commit ID.
    #[builder(into)]
    #[serde(rename = "lastCommitId")]
    pub r#last_commit_id: Option<String>,
    /// Specifies the name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: String,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceGithubRepo {
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
                "account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_name,
                )
                .await,
            );
            map.insert(
                "branch_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#branch_name,
                )
                .await,
            );
            map.insert(
                "git_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#git_url,
                )
                .await,
            );
            map.insert(
                "last_commit_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_commit_id,
                )
                .await,
            );
            map.insert(
                "repository_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repository_name,
                )
                .await,
            );
            map.insert(
                "root_folder".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_folder,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkspaceGithubRepo {
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
                    r#account_name: {
                        let field_value = match fields_map.get("account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#branch_name: {
                        let field_value = match fields_map.get("branch_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'branch_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_url: {
                        let field_value = match fields_map.get("git_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_commit_id: {
                        let field_value = match fields_map.get("last_commit_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_commit_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository_name: {
                        let field_value = match fields_map.get("repository_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_folder: {
                        let field_value = match fields_map.get("root_folder") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_folder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
