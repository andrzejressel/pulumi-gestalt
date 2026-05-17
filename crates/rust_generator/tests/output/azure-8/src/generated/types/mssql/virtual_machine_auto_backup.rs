#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAutoBackup {
    /// Enable or disable encryption for backups. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "encryptionEnabled")]
    pub r#encryption_enabled: Option<bool>,
    /// Encryption password to use. Must be specified when encryption is enabled.
    #[builder(into)]
    #[serde(rename = "encryptionPassword")]
    pub r#encryption_password: Option<String>,
    /// A `manual_schedule` block as documented below. When this block is present, the schedule type is set to `Manual`. Without this block, the schedule type is set to `Automated`.
    #[builder(into)]
    #[serde(rename = "manualSchedule")]
    pub r#manual_schedule: Option<Box<super::super::types::mssql::VirtualMachineAutoBackupManualSchedule>>,
    /// Retention period of backups, in days. Valid values are from `1` to `30`.
    #[builder(into)]
    #[serde(rename = "retentionPeriodInDays")]
    pub r#retention_period_in_days: i32,
    /// Access key for the storage account where backups will be kept.
    #[builder(into)]
    #[serde(rename = "storageAccountAccessKey")]
    pub r#storage_account_access_key: String,
    /// Blob endpoint for the storage account where backups will be kept.
    #[builder(into)]
    #[serde(rename = "storageBlobEndpoint")]
    pub r#storage_blob_endpoint: String,
    /// Include or exclude system databases from auto backup.
    #[builder(into)]
    #[serde(rename = "systemDatabasesBackupEnabled")]
    pub r#system_databases_backup_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAutoBackup {
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
                "encryption_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_enabled,
                )
                .await,
            );
            map.insert(
                "encryption_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_password,
                )
                .await,
            );
            map.insert(
                "manual_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#manual_schedule,
                )
                .await,
            );
            map.insert(
                "retention_period_in_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_period_in_days,
                )
                .await,
            );
            map.insert(
                "storage_account_access_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_access_key,
                )
                .await,
            );
            map.insert(
                "storage_blob_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_blob_endpoint,
                )
                .await,
            );
            map.insert(
                "system_databases_backup_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#system_databases_backup_enabled,
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
                        let field_value = match fields_map.get("encryption_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_password: {
                        let field_value = match fields_map.get("encryption_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_schedule: {
                        let field_value = match fields_map.get("manual_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'manual_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_period_in_days: {
                        let field_value = match fields_map.get("retention_period_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_period_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_access_key: {
                        let field_value = match fields_map.get("storage_account_access_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_access_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_blob_endpoint: {
                        let field_value = match fields_map.get("storage_blob_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_blob_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_databases_backup_enabled: {
                        let field_value = match fields_map.get("system_databases_backup_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'system_databases_backup_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
