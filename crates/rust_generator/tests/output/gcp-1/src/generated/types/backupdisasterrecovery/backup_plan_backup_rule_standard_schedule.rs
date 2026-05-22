#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupRuleStandardSchedule {
    /// A BackupWindow defines the window of the day during which backup jobs will run. Jobs are queued at the beginning of the window and will be marked as
    /// `NOT_RUN` if they do not start by the end of the window.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "backupWindow")]
    pub r#backup_window: Option<Box<super::super::types::backupdisasterrecovery::BackupPlanBackupRuleStandardScheduleBackupWindow>>,
    /// Specifies days of months like 1, 5, or 14 on which jobs will run.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Option<Vec<i32>>,
    /// Specifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for `recurrence_type`, `WEEKLY` and is not applicable otherwise.
    /// Each value may be one of: `DAY_OF_WEEK_UNSPECIFIED`, `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Specifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.
    /// This is required for `recurrence_type`, `HOURLY` and is not applicable otherwise.
    #[builder(into)]
    #[serde(rename = "hourlyFrequency")]
    pub r#hourly_frequency: Option<i32>,
    /// Specifies values of months
    /// Each value may be one of: `MONTH_UNSPECIFIED`, `JANUARY`, `FEBRUARY`, `MARCH`, `APRIL`, `MAY`, `JUNE`, `JULY`, `AUGUST`, `SEPTEMBER`, `OCTOBER`, `NOVEMBER`, `DECEMBER`.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Option<Vec<String>>,
    /// RecurrenceType enumerates the applicable periodicity for the schedule.
    /// Possible values are: `HOURLY`, `DAILY`, `WEEKLY`, `MONTHLY`, `YEARLY`.
    #[builder(into)]
    #[serde(rename = "recurrenceType")]
    pub r#recurrence_type: String,
    /// The time zone to be used when interpreting the schedule.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// Specifies a week day of the month like FIRST SUNDAY or LAST MONDAY, on which jobs will run.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weekDayOfMonth")]
    pub r#week_day_of_month: Option<Box<super::super::types::backupdisasterrecovery::BackupPlanBackupRuleStandardScheduleWeekDayOfMonth>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanBackupRuleStandardSchedule {
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
                    "backupWindow",
                    &self.r#backup_window,
                ),
                to_pulumi_object_field(
                    "daysOfMonths",
                    &self.r#days_of_months,
                ),
                to_pulumi_object_field(
                    "daysOfWeeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "hourlyFrequency",
                    &self.r#hourly_frequency,
                ),
                to_pulumi_object_field(
                    "months",
                    &self.r#months,
                ),
                to_pulumi_object_field(
                    "recurrenceType",
                    &self.r#recurrence_type,
                ),
                to_pulumi_object_field(
                    "timeZone",
                    &self.r#time_zone,
                ),
                to_pulumi_object_field(
                    "weekDayOfMonth",
                    &self.r#week_day_of_month,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanBackupRuleStandardSchedule {
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
                        let field_value = match fields_map.get("backupWindow") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupWindow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_months: {
                        let field_value = match fields_map.get("daysOfMonths") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysOfMonths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("daysOfWeeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysOfWeeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hourly_frequency: {
                        let field_value = match fields_map.get("hourlyFrequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'hourlyFrequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("recurrenceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurrenceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("timeZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#week_day_of_month: {
                        let field_value = match fields_map.get("weekDayOfMonth") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekDayOfMonth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
