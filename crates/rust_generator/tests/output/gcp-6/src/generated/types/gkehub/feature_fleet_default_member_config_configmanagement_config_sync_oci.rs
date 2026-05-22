#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci {
    /// The Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount
    #[builder(into)]
    #[serde(rename = "gcpServiceAccountEmail")]
    pub r#gcp_service_account_email: Option<String>,
    /// The absolute path of the directory that contains the local resources. Default: the root directory of the image
    #[builder(into)]
    #[serde(rename = "policyDir")]
    pub r#policy_dir: Option<String>,
    /// Type of secret configured for access to the Git repo
    #[builder(into)]
    #[serde(rename = "secretType")]
    pub r#secret_type: String,
    /// The OCI image repository URL for the package to sync from
    #[builder(into)]
    #[serde(rename = "syncRepo")]
    pub r#sync_repo: Option<String>,
    /// Period in seconds between consecutive syncs. Default: 15
    #[builder(into)]
    #[serde(rename = "syncWaitSecs")]
    pub r#sync_wait_secs: Option<String>,
    /// (Optional, Deprecated)
    /// Version of Config Sync installed
    /// 
    /// > **Warning:** The `configmanagement.config_sync.oci.version` field is deprecated and will be removed in a future major release. Please use `configmanagement.version` field to specify the version of Config Sync installed instead.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci {
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
                    "gcpServiceAccountEmail",
                    &self.r#gcp_service_account_email,
                ),
                to_pulumi_object_field(
                    "policyDir",
                    &self.r#policy_dir,
                ),
                to_pulumi_object_field(
                    "secretType",
                    &self.r#secret_type,
                ),
                to_pulumi_object_field(
                    "syncRepo",
                    &self.r#sync_repo,
                ),
                to_pulumi_object_field(
                    "syncWaitSecs",
                    &self.r#sync_wait_secs,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci {
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
                        let field_value = match fields_map.get("gcpServiceAccountEmail") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcpServiceAccountEmail' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_dir: {
                        let field_value = match fields_map.get("policyDir") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyDir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_type: {
                        let field_value = match fields_map.get("secretType") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_repo: {
                        let field_value = match fields_map.get("syncRepo") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncRepo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_wait_secs: {
                        let field_value = match fields_map.get("syncWaitSecs") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncWaitSecs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
