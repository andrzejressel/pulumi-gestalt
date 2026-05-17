#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstanceSettingBackupConfiguration {
    #[builder(into)]
    #[serde(rename = "backupRetentionSettings")]
    pub r#backup_retention_settings: Vec<super::super::types::sql::GetDatabaseInstanceSettingBackupConfigurationBackupRetentionSetting>,
    /// True if binary logging is enabled. If settings.backup_configuration.enabled is false, this must be as well. Can only be used with MySQL.
    #[builder(into)]
    #[serde(rename = "binaryLogEnabled")]
    pub r#binary_log_enabled: bool,
    /// True if backup configuration is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Location of the backup configuration.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// True if Point-in-time recovery is enabled.
    #[builder(into)]
    #[serde(rename = "pointInTimeRecoveryEnabled")]
    pub r#point_in_time_recovery_enabled: bool,
    /// HH:MM format time indicating when backup configuration starts.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7. (For PostgreSQL Enterprise Plus instances, from 1 to 35.)
    #[builder(into)]
    #[serde(rename = "transactionLogRetentionDays")]
    pub r#transaction_log_retention_days: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstanceSettingBackupConfiguration {
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
                "backup_retention_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_retention_settings,
                )
                .await,
            );
            map.insert(
                "binary_log_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#binary_log_enabled,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location,
                )
                .await,
            );
            map.insert(
                "point_in_time_recovery_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#point_in_time_recovery_enabled,
                )
                .await,
            );
            map.insert(
                "start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time,
                )
                .await,
            );
            map.insert(
                "transaction_log_retention_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transaction_log_retention_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstanceSettingBackupConfiguration {
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
                    r#backup_retention_settings: {
                        let field_value = match fields_map.get("backup_retention_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_retention_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#binary_log_enabled: {
                        let field_value = match fields_map.get("binary_log_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'binary_log_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#point_in_time_recovery_enabled: {
                        let field_value = match fields_map.get("point_in_time_recovery_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'point_in_time_recovery_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transaction_log_retention_days: {
                        let field_value = match fields_map.get("transaction_log_retention_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'transaction_log_retention_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
