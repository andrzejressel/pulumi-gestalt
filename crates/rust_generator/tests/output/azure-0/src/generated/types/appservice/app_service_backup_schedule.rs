#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppServiceBackupSchedule {
    /// Sets how often the backup should be executed.
    #[builder(into)]
    #[serde(rename = "frequencyInterval")]
    pub r#frequency_interval: i32,
    /// Sets the unit of time for how often the backup should be executed. Possible values are `Day` or `Hour`.
    #[builder(into)]
    #[serde(rename = "frequencyUnit")]
    pub r#frequency_unit: String,
    /// Should at least one backup always be kept in the Storage Account by the Retention Policy, regardless of how old it is?
    #[builder(into)]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub r#keep_at_least_one_backup: Option<bool>,
    /// Specifies the number of days after which Backups should be deleted. Defaults to `30`.
    #[builder(into)]
    #[serde(rename = "retentionPeriodInDays")]
    pub r#retention_period_in_days: Option<i32>,
    /// Sets when the schedule should start working.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppServiceBackupSchedule {
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
                "frequency_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frequency_interval,
                )
                .await,
            );
            map.insert(
                "frequency_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frequency_unit,
                )
                .await,
            );
            map.insert(
                "keep_at_least_one_backup".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#keep_at_least_one_backup,
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
                "start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppServiceBackupSchedule {
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
                    r#frequency_interval: {
                        let field_value = match fields_map.get("frequency_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frequency_unit: {
                        let field_value = match fields_map.get("frequency_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keep_at_least_one_backup: {
                        let field_value = match fields_map.get("keep_at_least_one_backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'keep_at_least_one_backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
