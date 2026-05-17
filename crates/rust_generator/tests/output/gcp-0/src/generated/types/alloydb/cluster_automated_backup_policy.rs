#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterAutomatedBackupPolicy {
    /// The length of the time window during which a backup can be taken. If a backup does not succeed within this time window, it will be canceled and considered failed.
    /// The backup window must be at least 5 minutes long. There is no upper bound on the window. If not set, it will default to 1 hour.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "backupWindow")]
    pub r#backup_window: Option<String>,
    /// Whether automated backups are enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyEncryptionConfig>>,
    /// Labels to apply to backups created using this configuration.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// The location where the backup will be stored. Currently, the only supported option is to store the backup in the same region as the cluster.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Quantity-based Backup retention policy to retain recent backups. Conflicts with 'time_based_retention', both can't be set together.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "quantityBasedRetention")]
    pub r#quantity_based_retention: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyQuantityBasedRetention>>,
    /// Time-based Backup retention policy. Conflicts with 'quantity_based_retention', both can't be set together.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeBasedRetention")]
    pub r#time_based_retention: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyTimeBasedRetention>>,
    /// Weekly schedule for the Backup.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyWeeklySchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterAutomatedBackupPolicy {
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
                "backup_window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_window,
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
                "encryption_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_config,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
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
                "quantity_based_retention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#quantity_based_retention,
                )
                .await,
            );
            map.insert(
                "time_based_retention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_based_retention,
                )
                .await,
            );
            map.insert(
                "weekly_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekly_schedule,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterAutomatedBackupPolicy {
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
                    r#backup_window: {
                        let field_value = match fields_map.get("backup_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#encryption_config: {
                        let field_value = match fields_map.get("encryption_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#quantity_based_retention: {
                        let field_value = match fields_map.get("quantity_based_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'quantity_based_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_based_retention: {
                        let field_value = match fields_map.get("time_based_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_based_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_schedule: {
                        let field_value = match fields_map.get("weekly_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
