#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSourceBackupConfigInfo {
    /// Configuration for an application backed up by a Backup Appliance.
    #[builder(into)]
    #[serde(rename = "backupApplianceBackupConfigs")]
    pub r#backup_appliance_backup_configs: Vec<super::super::types::backupdisasterrecovery::GetDataSourceBackupConfigInfoBackupApplianceBackupConfig>,
    /// Configuration for a Google Cloud resource.
    #[builder(into)]
    #[serde(rename = "gcpBackupConfigs")]
    pub r#gcp_backup_configs: Vec<super::super::types::backupdisasterrecovery::GetDataSourceBackupConfigInfoGcpBackupConfig>,
    /// If the last backup failed, this field has the error message.
    #[builder(into)]
    #[serde(rename = "lastBackupError")]
    pub r#last_backup_error: std::collections::HashMap<String, String>,
    /// LastBackupstate tracks whether the last backup was not yet started, successful, failed, or could not be run because of the lack of permissions.
    #[builder(into)]
    #[serde(rename = "lastBackupState")]
    pub r#last_backup_state: String,
    /// If the last backup were successful, this field has the consistency date.
    #[builder(into)]
    #[serde(rename = "lastSuccessfulBackupConsistencyTime")]
    pub r#last_successful_backup_consistency_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSourceBackupConfigInfo {
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
                "backup_appliance_backup_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_appliance_backup_configs,
                )
                .await,
            );
            map.insert(
                "gcp_backup_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcp_backup_configs,
                )
                .await,
            );
            map.insert(
                "last_backup_error".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_backup_error,
                )
                .await,
            );
            map.insert(
                "last_backup_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_backup_state,
                )
                .await,
            );
            map.insert(
                "last_successful_backup_consistency_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_successful_backup_consistency_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSourceBackupConfigInfo {
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
                    r#backup_appliance_backup_configs: {
                        let field_value = match fields_map.get("backup_appliance_backup_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_appliance_backup_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_backup_configs: {
                        let field_value = match fields_map.get("gcp_backup_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcp_backup_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_backup_error: {
                        let field_value = match fields_map.get("last_backup_error") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_backup_error' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_backup_state: {
                        let field_value = match fields_map.get("last_backup_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_backup_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_successful_backup_consistency_time: {
                        let field_value = match fields_map.get("last_successful_backup_consistency_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_successful_backup_consistency_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
