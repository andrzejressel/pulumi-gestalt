#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipConfigmanagementConfigSync {
    /// Enables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// (Optional) Structure is documented below.
    #[builder(into)]
    #[serde(rename = "git")]
    pub r#git: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementConfigSyncGit>>,
    /// Deprecated: If Workload Identity Federation for GKE is enabled, Google Cloud Service Account is no longer needed for exporting Config Sync metrics: https://cloud.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/monitor-config-sync-cloud-monitoring#custom-monitoring.
    #[builder(into)]
    #[serde(rename = "metricsGcpServiceAccountEmail")]
    pub r#metrics_gcp_service_account_email: Option<String>,
    /// (Optional) Supported from Config Sync versions 1.12.0 onwards. Structure is documented below.
    /// 
    /// Use either `git` or `oci` config option.
    #[builder(into)]
    #[serde(rename = "oci")]
    pub r#oci: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementConfigSyncOci>>,
    /// Supported from Config Sync versions 1.10.0 onwards. Set to `true` to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts.
    #[builder(into)]
    #[serde(rename = "preventDrift")]
    pub r#prevent_drift: Option<bool>,
    /// Specifies whether the Config Sync Repo is in "hierarchical" or "unstructured" mode.
    #[builder(into)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Option<String>,
    /// Set to `true` to stop syncing configurations for a single cluster. This field is only available on clusters using Config Sync [auto-upgrades](http://cloud/kubernetes-engine/enterprise/config-sync/docs/how-to/upgrade-config-sync#auto-upgrade-config) or on Config Sync version 1.20.0 or later. Defaults: `false`.
    #[builder(into)]
    #[serde(rename = "stopSyncing")]
    pub r#stop_syncing: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureMembershipConfigmanagementConfigSync {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "git",
                    &self.r#git,
                ),
                to_pulumi_object_field(
                    "metrics_gcp_service_account_email",
                    &self.r#metrics_gcp_service_account_email,
                ),
                to_pulumi_object_field(
                    "oci",
                    &self.r#oci,
                ),
                to_pulumi_object_field(
                    "prevent_drift",
                    &self.r#prevent_drift,
                ),
                to_pulumi_object_field(
                    "source_format",
                    &self.r#source_format,
                ),
                to_pulumi_object_field(
                    "stop_syncing",
                    &self.r#stop_syncing,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureMembershipConfigmanagementConfigSync {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git: {
                        let field_value = match fields_map.get("git") {
                            Some(value) => value,
                            None => bail!("Missing field 'git' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metrics_gcp_service_account_email: {
                        let field_value = match fields_map.get("metrics_gcp_service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'metrics_gcp_service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oci: {
                        let field_value = match fields_map.get("oci") {
                            Some(value) => value,
                            None => bail!("Missing field 'oci' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prevent_drift: {
                        let field_value = match fields_map.get("prevent_drift") {
                            Some(value) => value,
                            None => bail!("Missing field 'prevent_drift' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_format: {
                        let field_value = match fields_map.get("source_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stop_syncing: {
                        let field_value = match fields_map.get("stop_syncing") {
                            Some(value) => value,
                            None => bail!("Missing field 'stop_syncing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
