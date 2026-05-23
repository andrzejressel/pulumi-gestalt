#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsBackupConfiguration {
    /// Backup retention settings. The configuration is detailed below.
    #[builder(into)]
    pub r#backup_retention_settings: Option<Box<super::super::types::sql::DatabaseInstanceSettingsBackupConfigurationBackupRetentionSettings>>,
    /// True if binary logging is enabled.
    /// Can only be used with MySQL.
    #[builder(into)]
    pub r#binary_log_enabled: Option<bool>,
    /// True if backup configuration is enabled.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// The region where the backup will be stored
    #[builder(into)]
    pub r#location: Option<String>,
    /// True if Point-in-time recovery is enabled. Will restart database if enabled after instance creation. Valid only for PostgreSQL and SQL Server instances.
    #[builder(into)]
    pub r#point_in_time_recovery_enabled: Option<bool>,
    /// `HH:MM` format time indicating when backup
    /// configuration starts.
    #[builder(into)]
    pub r#start_time: Option<String>,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7. For PostgreSQL Enterprise Plus instances, the number of days of retained transaction logs can be set from 1 to 35.
    #[builder(into)]
    pub r#transaction_log_retention_days: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceSettingsBackupConfiguration {
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
                    "backupRetentionSettings",
                    &self.r#backup_retention_settings,
                ),
                to_pulumi_object_field(
                    "binaryLogEnabled",
                    &self.r#binary_log_enabled,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "pointInTimeRecoveryEnabled",
                    &self.r#point_in_time_recovery_enabled,
                ),
                to_pulumi_object_field(
                    "startTime",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "transactionLogRetentionDays",
                    &self.r#transaction_log_retention_days,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceSettingsBackupConfiguration {
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
                        let field_value = match fields_map.get("backupRetentionSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupRetentionSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#binary_log_enabled: {
                        let field_value = match fields_map.get("binaryLogEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'binaryLogEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("pointInTimeRecoveryEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'pointInTimeRecoveryEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("startTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'startTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transaction_log_retention_days: {
                        let field_value = match fields_map.get("transactionLogRetentionDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'transactionLogRetentionDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
