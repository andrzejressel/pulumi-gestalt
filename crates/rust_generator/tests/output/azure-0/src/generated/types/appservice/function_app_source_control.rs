#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionAppSourceControl {
    /// The branch of the remote repository to use. Defaults to 'master'.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// Limits to manual integration. Defaults to `false` if not specified.
    #[builder(into)]
    #[serde(rename = "manualIntegration")]
    pub r#manual_integration: Option<bool>,
    /// The URL of the source code repository.
    #[builder(into)]
    #[serde(rename = "repoUrl")]
    pub r#repo_url: Option<String>,
    /// Enable roll-back for the repository. Defaults to `false` if not specified.
    #[builder(into)]
    #[serde(rename = "rollbackEnabled")]
    pub r#rollback_enabled: Option<bool>,
    /// Use Mercurial if `true`, otherwise uses Git.
    #[builder(into)]
    #[serde(rename = "useMercurial")]
    pub r#use_mercurial: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionAppSourceControl {
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
                "branch".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#branch,
                )
                .await,
            );
            map.insert(
                "manual_integration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#manual_integration,
                )
                .await,
            );
            map.insert(
                "repo_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repo_url,
                )
                .await,
            );
            map.insert(
                "rollback_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rollback_enabled,
                )
                .await,
            );
            map.insert(
                "use_mercurial".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_mercurial,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionAppSourceControl {
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
                    r#branch: {
                        let field_value = match fields_map.get("branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_integration: {
                        let field_value = match fields_map.get("manual_integration") {
                            Some(value) => value,
                            None => bail!("Missing field 'manual_integration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repo_url: {
                        let field_value = match fields_map.get("repo_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'repo_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rollback_enabled: {
                        let field_value = match fields_map.get("rollback_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'rollback_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_mercurial: {
                        let field_value = match fields_map.get("use_mercurial") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_mercurial' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
