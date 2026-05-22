#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FactoryVstsConfiguration {
    /// Specifies the VSTS account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// Specifies the branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// Specifies the name of the VSTS project.
    #[builder(into)]
    #[serde(rename = "projectName")]
    pub r#project_name: String,
    /// Is automated publishing enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "publishingEnabled")]
    pub r#publishing_enabled: Option<bool>,
    /// Specifies the name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: String,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: String,
    /// Specifies the Tenant ID associated with the VSTS account.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FactoryVstsConfiguration {
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
                    "projectName",
                    &self.r#project_name,
                ),
                to_pulumi_object_field(
                    "publishingEnabled",
                    &self.r#publishing_enabled,
                ),
                to_pulumi_object_field(
                    "repositoryName",
                    &self.r#repository_name,
                ),
                to_pulumi_object_field(
                    "rootFolder",
                    &self.r#root_folder,
                ),
                to_pulumi_object_field(
                    "tenantId",
                    &self.r#tenant_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FactoryVstsConfiguration {
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
                    r#project_name: {
                        let field_value = match fields_map.get("projectName") {
                            Some(value) => value,
                            None => bail!("Missing field 'projectName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publishing_enabled: {
                        let field_value = match fields_map.get("publishingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'publishingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenantId") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenantId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
