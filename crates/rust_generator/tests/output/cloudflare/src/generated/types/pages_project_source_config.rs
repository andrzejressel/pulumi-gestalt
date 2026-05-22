#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PagesProjectSourceConfig {
    /// Toggle deployments on this repo. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "deploymentsEnabled")]
    pub r#deployments_enabled: Option<bool>,
    /// Project owner username. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Option<String>,
    /// Enable Pages to comment on Pull Requests. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "prCommentsEnabled")]
    pub r#pr_comments_enabled: Option<bool>,
    /// Branches will be excluded from automatic deployment.
    #[builder(into)]
    #[serde(rename = "previewBranchExcludes")]
    pub r#preview_branch_excludes: Option<Vec<String>>,
    /// Branches will be included for automatic deployment.
    #[builder(into)]
    #[serde(rename = "previewBranchIncludes")]
    pub r#preview_branch_includes: Option<Vec<String>>,
    /// Preview Deployment Setting. Available values: `custom`, `all`, `none`. Defaults to `all`.
    #[builder(into)]
    #[serde(rename = "previewDeploymentSetting")]
    pub r#preview_deployment_setting: Option<String>,
    /// Project production branch name.
    #[builder(into)]
    #[serde(rename = "productionBranch")]
    pub r#production_branch: String,
    /// Enable production deployments. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "productionDeploymentEnabled")]
    pub r#production_deployment_enabled: Option<bool>,
    /// Project repository name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    #[serde(rename = "repoName")]
    pub r#repo_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PagesProjectSourceConfig {
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
                    "deploymentsEnabled",
                    &self.r#deployments_enabled,
                ),
                to_pulumi_object_field(
                    "owner",
                    &self.r#owner,
                ),
                to_pulumi_object_field(
                    "prCommentsEnabled",
                    &self.r#pr_comments_enabled,
                ),
                to_pulumi_object_field(
                    "previewBranchExcludes",
                    &self.r#preview_branch_excludes,
                ),
                to_pulumi_object_field(
                    "previewBranchIncludes",
                    &self.r#preview_branch_includes,
                ),
                to_pulumi_object_field(
                    "previewDeploymentSetting",
                    &self.r#preview_deployment_setting,
                ),
                to_pulumi_object_field(
                    "productionBranch",
                    &self.r#production_branch,
                ),
                to_pulumi_object_field(
                    "productionDeploymentEnabled",
                    &self.r#production_deployment_enabled,
                ),
                to_pulumi_object_field(
                    "repoName",
                    &self.r#repo_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PagesProjectSourceConfig {
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
                    r#deployments_enabled: {
                        let field_value = match fields_map.get("deploymentsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploymentsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#owner: {
                        let field_value = match fields_map.get("owner") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pr_comments_enabled: {
                        let field_value = match fields_map.get("prCommentsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'prCommentsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview_branch_excludes: {
                        let field_value = match fields_map.get("previewBranchExcludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'previewBranchExcludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview_branch_includes: {
                        let field_value = match fields_map.get("previewBranchIncludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'previewBranchIncludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview_deployment_setting: {
                        let field_value = match fields_map.get("previewDeploymentSetting") {
                            Some(value) => value,
                            None => bail!("Missing field 'previewDeploymentSetting' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#production_branch: {
                        let field_value = match fields_map.get("productionBranch") {
                            Some(value) => value,
                            None => bail!("Missing field 'productionBranch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#production_deployment_enabled: {
                        let field_value = match fields_map.get("productionDeploymentEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'productionDeploymentEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repo_name: {
                        let field_value = match fields_map.get("repoName") {
                            Some(value) => value,
                            None => bail!("Missing field 'repoName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
