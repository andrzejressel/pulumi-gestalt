#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupSchedule {
    /// A standard cron string that defines a repeating schedule for
    /// creating Backups via this BackupPlan.
    /// This is mutually exclusive with the rpoConfig field since at most one
    /// schedule can be defined for a BackupPlan.
    /// If this is defined, then backupRetainDays must also be defined.
    #[builder(into)]
    #[serde(rename = "cronSchedule")]
    pub r#cron_schedule: Option<String>,
    /// This flag denotes whether automatic Backup creation is paused for this BackupPlan.
    #[builder(into)]
    #[serde(rename = "paused")]
    pub r#paused: Option<bool>,
    /// Defines the RPO schedule configuration for this BackupPlan. This is mutually
    /// exclusive with the cronSchedule field since at most one schedule can be defined
    /// for a BackupPLan. If this is defined, then backupRetainDays must also be defined.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rpoConfig")]
    pub r#rpo_config: Option<Box<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanBackupSchedule {
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
                "cron_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cron_schedule,
                )
                .await,
            );
            map.insert(
                "paused".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#paused,
                )
                .await,
            );
            map.insert(
                "rpo_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rpo_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanBackupSchedule {
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
                    r#cron_schedule: {
                        let field_value = match fields_map.get("cron_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'cron_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#paused: {
                        let field_value = match fields_map.get("paused") {
                            Some(value) => value,
                            None => bail!("Missing field 'paused' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rpo_config: {
                        let field_value = match fields_map.get("rpo_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'rpo_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
