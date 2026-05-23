#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAutoBackup {
    /// Enable or disable encryption for backups. Defaults to `false`.
    #[builder(into)]
    pub r#encryption_enabled: Option<bool>,
    /// Encryption password to use. Must be specified when encryption is enabled.
    #[builder(into)]
    pub r#encryption_password: Option<String>,
    /// A `manual_schedule` block as documented below. When this block is present, the schedule type is set to `Manual`. Without this block, the schedule type is set to `Automated`.
    #[builder(into)]
    pub r#manual_schedule: Option<Box<super::super::types::mssql::VirtualMachineAutoBackupManualSchedule>>,
    /// Retention period of backups, in days. Valid values are from `1` to `30`.
    #[builder(into)]
    pub r#retention_period_in_days: i32,
    /// Access key for the storage account where backups will be kept.
    #[builder(into)]
    pub r#storage_account_access_key: String,
    /// Blob endpoint for the storage account where backups will be kept.
    #[builder(into)]
    pub r#storage_blob_endpoint: String,
    /// Include or exclude system databases from auto backup.
    #[builder(into)]
    pub r#system_databases_backup_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAutoBackup {
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
                    "encryptionEnabled",
                    &self.r#encryption_enabled,
                ),
                to_pulumi_object_field(
                    "encryptionPassword",
                    &self.r#encryption_password,
                ),
                to_pulumi_object_field(
                    "manualSchedule",
                    &self.r#manual_schedule,
                ),
                to_pulumi_object_field(
                    "retentionPeriodInDays",
                    &self.r#retention_period_in_days,
                ),
                to_pulumi_object_field(
                    "storageAccountAccessKey",
                    &self.r#storage_account_access_key,
                ),
                to_pulumi_object_field(
                    "storageBlobEndpoint",
                    &self.r#storage_blob_endpoint,
                ),
                to_pulumi_object_field(
                    "systemDatabasesBackupEnabled",
                    &self.r#system_databases_backup_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineAutoBackup {
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
                    r#encryption_enabled: {
                        let field_value = match fields_map.get("encryptionEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_password: {
                        let field_value = match fields_map.get("encryptionPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_schedule: {
                        let field_value = match fields_map.get("manualSchedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'manualSchedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_period_in_days: {
                        let field_value = match fields_map.get("retentionPeriodInDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'retentionPeriodInDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_access_key: {
                        let field_value = match fields_map.get("storageAccountAccessKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountAccessKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_blob_endpoint: {
                        let field_value = match fields_map.get("storageBlobEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageBlobEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_databases_backup_enabled: {
                        let field_value = match fields_map.get("systemDatabasesBackupEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'systemDatabasesBackupEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
