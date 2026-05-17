#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackupPlanBackupRuleStandardSchedule {
    /// A BackupWindow defines the window of the day during which backup jobs will run. Jobs are queued at the beginning of the window and will be marked as
    /// 'NOT_RUN' if they do not start by the end of the window.
    #[builder(into)]
    #[serde(rename = "backupWindows")]
    pub r#backup_windows: Vec<super::super::types::backupdisasterrecovery::GetBackupPlanBackupRuleStandardScheduleBackupWindow>,
    /// Specifies days of months like 1, 5, or 14 on which jobs will run.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Vec<i32>,
    /// Specifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for 'recurrence_type', 'WEEKLY' and is not applicable otherwise. Possible values: ["DAY_OF_WEEK_UNSPECIFIED", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"]
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Vec<String>,
    /// Specifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.
    /// This is required for 'recurrence_type', 'HOURLY' and is not applicable otherwise.
    #[builder(into)]
    #[serde(rename = "hourlyFrequency")]
    pub r#hourly_frequency: i32,
    /// Specifies values of months Possible values: ["MONTH_UNSPECIFIED", "JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE", "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER"]
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Vec<String>,
    /// RecurrenceType enumerates the applicable periodicity for the schedule. Possible values: ["HOURLY", "DAILY", "WEEKLY", "MONTHLY", "YEARLY"]
    #[builder(into)]
    #[serde(rename = "recurrenceType")]
    pub r#recurrence_type: String,
    /// The time zone to be used when interpreting the schedule.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// Specifies a week day of the month like FIRST SUNDAY or LAST MONDAY, on which jobs will run.
    #[builder(into)]
    #[serde(rename = "weekDayOfMonths")]
    pub r#week_day_of_months: Vec<super::super::types::backupdisasterrecovery::GetBackupPlanBackupRuleStandardScheduleWeekDayOfMonth>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackupPlanBackupRuleStandardSchedule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backup_windows",
                    &self.r#backup_windows,
                ),
                to_pulumi_object_field(
                    "days_of_months",
                    &self.r#days_of_months,
                ),
                to_pulumi_object_field(
                    "days_of_weeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "hourly_frequency",
                    &self.r#hourly_frequency,
                ),
                to_pulumi_object_field(
                    "months",
                    &self.r#months,
                ),
                to_pulumi_object_field(
                    "recurrence_type",
                    &self.r#recurrence_type,
                ),
                to_pulumi_object_field(
                    "time_zone",
                    &self.r#time_zone,
                ),
                to_pulumi_object_field(
                    "week_day_of_months",
                    &self.r#week_day_of_months,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackupPlanBackupRuleStandardSchedule {
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
                    r#backup_windows: {
                        let field_value = match fields_map.get("backup_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_months: {
                        let field_value = match fields_map.get("days_of_months") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_of_months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("days_of_weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_of_weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hourly_frequency: {
                        let field_value = match fields_map.get("hourly_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'hourly_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#months: {
                        let field_value = match fields_map.get("months") {
                            Some(value) => value,
                            None => bail!("Missing field 'months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recurrence_type: {
                        let field_value = match fields_map.get("recurrence_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurrence_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#week_day_of_months: {
                        let field_value = match fields_map.get("week_day_of_months") {
                            Some(value) => value,
                            None => bail!("Missing field 'week_day_of_months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
