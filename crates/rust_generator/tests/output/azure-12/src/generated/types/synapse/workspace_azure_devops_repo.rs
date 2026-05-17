#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceAzureDevopsRepo {
    /// Specifies the Azure DevOps account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// Specifies the collaboration branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// The last commit ID.
    #[builder(into)]
    #[serde(rename = "lastCommitId")]
    pub r#last_commit_id: Option<String>,
    /// Specifies the name of the Azure DevOps project.
    #[builder(into)]
    #[serde(rename = "projectName")]
    pub r#project_name: String,
    /// Specifies the name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: String,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: String,
    /// the ID of the tenant for the Azure DevOps account.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceAzureDevopsRepo {
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
                    "account_name",
                    &self.r#account_name,
                ),
                to_pulumi_object_field(
                    "branch_name",
                    &self.r#branch_name,
                ),
                to_pulumi_object_field(
                    "last_commit_id",
                    &self.r#last_commit_id,
                ),
                to_pulumi_object_field(
                    "project_name",
                    &self.r#project_name,
                ),
                to_pulumi_object_field(
                    "repository_name",
                    &self.r#repository_name,
                ),
                to_pulumi_object_field(
                    "root_folder",
                    &self.r#root_folder,
                ),
                to_pulumi_object_field(
                    "tenant_id",
                    &self.r#tenant_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkspaceAzureDevopsRepo {
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
                    r#last_commit_id: {
                        let field_value = match fields_map.get("last_commit_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_commit_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_name: {
                        let field_value = match fields_map.get("project_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
