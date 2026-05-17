#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipConfigmanagementConfigSyncGit {
    /// The GCP Service Account Email used for auth when secretType is gcpServiceAccount.
    #[builder(into)]
    #[serde(rename = "gcpServiceAccountEmail")]
    pub r#gcp_service_account_email: Option<String>,
    /// URL for the HTTPS proxy to be used when communicating with the Git repo.
    #[builder(into)]
    #[serde(rename = "httpsProxy")]
    pub r#https_proxy: Option<String>,
    /// The path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository.
    #[builder(into)]
    #[serde(rename = "policyDir")]
    pub r#policy_dir: Option<String>,
    /// Type of secret configured for access to the Git repo.
    #[builder(into)]
    #[serde(rename = "secretType")]
    pub r#secret_type: Option<String>,
    /// The branch of the repository to sync from. Default: master.
    #[builder(into)]
    #[serde(rename = "syncBranch")]
    pub r#sync_branch: Option<String>,
    /// The URL of the Git repository to use as the source of truth.
    #[builder(into)]
    #[serde(rename = "syncRepo")]
    pub r#sync_repo: Option<String>,
    /// Git revision (tag or hash) to check out. Default HEAD.
    #[builder(into)]
    #[serde(rename = "syncRev")]
    pub r#sync_rev: Option<String>,
    /// Period in seconds between consecutive syncs. Default: 15.
    #[builder(into)]
    #[serde(rename = "syncWaitSecs")]
    pub r#sync_wait_secs: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureMembershipConfigmanagementConfigSyncGit {
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
                "gcp_service_account_email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcp_service_account_email,
                )
                .await,
            );
            map.insert(
                "https_proxy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#https_proxy,
                )
                .await,
            );
            map.insert(
                "policy_dir".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_dir,
                )
                .await,
            );
            map.insert(
                "secret_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_type,
                )
                .await,
            );
            map.insert(
                "sync_branch".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_branch,
                )
                .await,
            );
            map.insert(
                "sync_repo".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_repo,
                )
                .await,
            );
            map.insert(
                "sync_rev".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_rev,
                )
                .await,
            );
            map.insert(
                "sync_wait_secs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_wait_secs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureMembershipConfigmanagementConfigSyncGit {
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
                    r#gcp_service_account_email: {
                        let field_value = match fields_map.get("gcp_service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcp_service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_proxy: {
                        let field_value = match fields_map.get("https_proxy") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_proxy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_dir: {
                        let field_value = match fields_map.get("policy_dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_type: {
                        let field_value = match fields_map.get("secret_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_branch: {
                        let field_value = match fields_map.get("sync_branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_repo: {
                        let field_value = match fields_map.get("sync_repo") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_repo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_rev: {
                        let field_value = match fields_map.get("sync_rev") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_rev' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_wait_secs: {
                        let field_value = match fields_map.get("sync_wait_secs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_wait_secs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
