#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceGithubRepo {
    /// Specifies the GitHub account name.
    #[builder(into)]
    pub r#account_name: String,
    /// Specifies the collaboration branch of the repository to get code from.
    #[builder(into)]
    pub r#branch_name: String,
    /// Specifies the GitHub Enterprise host name. For example: <https://github.mydomain.com>.
    /// 
    /// > **Note:** You must log in to the Synapse UI to complete the authentication to the GitHub repository.
    #[builder(into)]
    pub r#git_url: Option<String>,
    /// The last commit ID.
    #[builder(into)]
    pub r#last_commit_id: Option<String>,
    /// Specifies the name of the git repository.
    #[builder(into)]
    pub r#repository_name: String,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    pub r#root_folder: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceGithubRepo {
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
                    "accountName",
                    &self.r#account_name,
                ),
                to_pulumi_object_field(
                    "branchName",
                    &self.r#branch_name,
                ),
                to_pulumi_object_field(
                    "gitUrl",
                    &self.r#git_url,
                ),
                to_pulumi_object_field(
                    "lastCommitId",
                    &self.r#last_commit_id,
                ),
                to_pulumi_object_field(
                    "repositoryName",
                    &self.r#repository_name,
                ),
                to_pulumi_object_field(
                    "rootFolder",
                    &self.r#root_folder,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("accountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#branch_name: {
                        let field_value = match fields_map.get("branchName") {
                            Some(value) => value,
                            None => bail!("Missing field 'branchName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_url: {
                        let field_value = match fields_map.get("gitUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'gitUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_commit_id: {
                        let field_value = match fields_map.get("lastCommitId") {
                            Some(value) => value,
                            None => bail!("Missing field 'lastCommitId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository_name: {
                        let field_value = match fields_map.get("repositoryName") {
                            Some(value) => value,
                            None => bail!("Missing field 'repositoryName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_folder: {
                        let field_value = match fields_map.get("rootFolder") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootFolder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
