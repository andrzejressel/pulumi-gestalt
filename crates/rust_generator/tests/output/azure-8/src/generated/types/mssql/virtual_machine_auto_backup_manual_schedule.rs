#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAutoBackupManualSchedule {
    /// A list of days on which backup can take place. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`
    /// 
    /// > **NOTE:** `days_of_week` can only be specified when `manual_schedule` is set to `Weekly`
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Frequency of full backups. Valid values include `Daily` or `Weekly`.
    #[builder(into)]
    #[serde(rename = "fullBackupFrequency")]
    pub r#full_backup_frequency: String,
    /// Start hour of a given day during which full backups can take place. Valid values are from `0` to `23`.
    #[builder(into)]
    #[serde(rename = "fullBackupStartHour")]
    pub r#full_backup_start_hour: i32,
    /// Duration of the time window of a given day during which full backups can take place, in hours. Valid values are between `1` and `23`.
    #[builder(into)]
    #[serde(rename = "fullBackupWindowInHours")]
    pub r#full_backup_window_in_hours: i32,
    /// Frequency of log backups, in minutes. Valid values are from `5` to `60`.
    #[builder(into)]
    #[serde(rename = "logBackupFrequencyInMinutes")]
    pub r#log_backup_frequency_in_minutes: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAutoBackupManualSchedule {
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
                "days_of_weeks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#days_of_weeks,
                )
                .await,
            );
            map.insert(
                "full_backup_frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#full_backup_frequency,
                )
                .await,
            );
            map.insert(
                "full_backup_start_hour".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#full_backup_start_hour,
                )
                .await,
            );
            map.insert(
                "full_backup_window_in_hours".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#full_backup_window_in_hours,
                )
                .await,
            );
            map.insert(
                "log_backup_frequency_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_backup_frequency_in_minutes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineAutoBackupManualSchedule {
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
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("days_of_weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_of_weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#full_backup_frequency: {
                        let field_value = match fields_map.get("full_backup_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'full_backup_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#full_backup_start_hour: {
                        let field_value = match fields_map.get("full_backup_start_hour") {
                            Some(value) => value,
                            None => bail!("Missing field 'full_backup_start_hour' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#full_backup_window_in_hours: {
                        let field_value = match fields_map.get("full_backup_window_in_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'full_backup_window_in_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_backup_frequency_in_minutes: {
                        let field_value = match fields_map.get("log_backup_frequency_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_backup_frequency_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
