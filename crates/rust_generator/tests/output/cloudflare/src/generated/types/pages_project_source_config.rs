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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "deployments_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deployments_enabled,
                )
                .await,
            );
            map.insert(
                "owner".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#owner,
                )
                .await,
            );
            map.insert(
                "pr_comments_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pr_comments_enabled,
                )
                .await,
            );
            map.insert(
                "preview_branch_excludes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preview_branch_excludes,
                )
                .await,
            );
            map.insert(
                "preview_branch_includes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preview_branch_includes,
                )
                .await,
            );
            map.insert(
                "preview_deployment_setting".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preview_deployment_setting,
                )
                .await,
            );
            map.insert(
                "production_branch".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#production_branch,
                )
                .await,
            );
            map.insert(
                "production_deployment_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#production_deployment_enabled,
                )
                .await,
            );
            map.insert(
                "repo_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repo_name,
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
                        let field_value = match fields_map.get("deployments_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployments_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("pr_comments_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'pr_comments_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview_branch_excludes: {
                        let field_value = match fields_map.get("preview_branch_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'preview_branch_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview_branch_includes: {
                        let field_value = match fields_map.get("preview_branch_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'preview_branch_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview_deployment_setting: {
                        let field_value = match fields_map.get("preview_deployment_setting") {
                            Some(value) => value,
                            None => bail!("Missing field 'preview_deployment_setting' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#production_branch: {
                        let field_value = match fields_map.get("production_branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'production_branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#production_deployment_enabled: {
                        let field_value = match fields_map.get("production_deployment_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'production_deployment_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repo_name: {
                        let field_value = match fields_map.get("repo_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'repo_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
