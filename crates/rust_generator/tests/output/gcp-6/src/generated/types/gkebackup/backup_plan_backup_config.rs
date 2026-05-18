#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupConfig {
    /// If True, include all namespaced resources.
    #[builder(into)]
    #[serde(rename = "allNamespaces")]
    pub r#all_namespaces: Option<bool>,
    /// This defines a customer managed encryption key that will be used to encrypt the "config"
    /// portion (the Kubernetes resources) of Backups created via this plan.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Option<Box<super::super::types::gkebackup::BackupPlanBackupConfigEncryptionKey>>,
    /// This flag specifies whether Kubernetes Secret resources should be included
    /// when they fall into the scope of Backups.
    #[builder(into)]
    #[serde(rename = "includeSecrets")]
    pub r#include_secrets: Option<bool>,
    /// This flag specifies whether volume data should be backed up when PVCs are
    /// included in the scope of a Backup.
    #[builder(into)]
    #[serde(rename = "includeVolumeData")]
    pub r#include_volume_data: Option<bool>,
    /// This flag specifies whether Backups will not fail when
    /// Backup for GKE detects Kubernetes configuration that is
    /// non-standard or requires additional setup to restore.
    #[builder(into)]
    #[serde(rename = "permissiveMode")]
    pub r#permissive_mode: Option<bool>,
    /// A list of namespaced Kubernetes Resources.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedApplications")]
    pub r#selected_applications: Option<Box<super::super::types::gkebackup::BackupPlanBackupConfigSelectedApplications>>,
    /// If set, include just the resources in the listed namespaces.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedNamespaces")]
    pub r#selected_namespaces: Option<Box<super::super::types::gkebackup::BackupPlanBackupConfigSelectedNamespaces>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanBackupConfig {
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
                    "all_namespaces",
                    &self.r#all_namespaces,
                ),
                to_pulumi_object_field(
                    "encryption_key",
                    &self.r#encryption_key,
                ),
                to_pulumi_object_field(
                    "include_secrets",
                    &self.r#include_secrets,
                ),
                to_pulumi_object_field(
                    "include_volume_data",
                    &self.r#include_volume_data,
                ),
                to_pulumi_object_field(
                    "permissive_mode",
                    &self.r#permissive_mode,
                ),
                to_pulumi_object_field(
                    "selected_applications",
                    &self.r#selected_applications,
                ),
                to_pulumi_object_field(
                    "selected_namespaces",
                    &self.r#selected_namespaces,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanBackupConfig {
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
                    r#all_namespaces: {
                        let field_value = match fields_map.get("all_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_key: {
                        let field_value = match fields_map.get("encryption_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_secrets: {
                        let field_value = match fields_map.get("include_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_volume_data: {
                        let field_value = match fields_map.get("include_volume_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_volume_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_mode: {
                        let field_value = match fields_map.get("permissive_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissive_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selected_applications: {
                        let field_value = match fields_map.get("selected_applications") {
                            Some(value) => value,
                            None => bail!("Missing field 'selected_applications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selected_namespaces: {
                        let field_value = match fields_map.get("selected_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'selected_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
