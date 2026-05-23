#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindow {
    /// The exclusion window occurs every day if set to "True".
    /// Specifying this field to "False" is an error.
    /// Only one of singleOccurrenceDate, daily and daysOfWeek may be set.
    #[builder(into)]
    pub r#daily: Option<bool>,
    /// The exclusion window occurs on these days of each week in UTC.
    /// Only one of singleOccurrenceDate, daily and daysOfWeek may be set.
    /// Structure is documented below.
    #[builder(into)]
    pub r#days_of_week: Option<Box<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindowDaysOfWeek>>,
    /// Specifies duration of the window in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s". Restrictions for duration based on the
    /// recurrence type to allow some time for backup to happen:
    /// - single_occurrence_date:  no restriction
    /// - daily window: duration < 24 hours
    /// - weekly window:
    /// - days of week includes all seven days of a week: duration < 24 hours
    /// - all other weekly window: duration < 168 hours (i.e., 24 * 7 hours)
    #[builder(into)]
    pub r#duration: String,
    /// No recurrence. The exclusion window occurs only once and on this date in UTC.
    /// Only one of singleOccurrenceDate, daily and daysOfWeek may be set.
    /// Structure is documented below.
    #[builder(into)]
    pub r#single_occurrence_date: Option<Box<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindowSingleOccurrenceDate>>,
    /// Specifies the start time of the window using time of the day in UTC.
    /// Structure is documented below.
    #[builder(into)]
    pub r#start_time: Box<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanBackupScheduleRpoConfigExclusionWindow {
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
                    "daily",
                    &self.r#daily,
                ),
                to_pulumi_object_field(
                    "daysOfWeek",
                    &self.r#days_of_week,
                ),
                to_pulumi_object_field(
                    "duration",
                    &self.r#duration,
                ),
                to_pulumi_object_field(
                    "singleOccurrenceDate",
                    &self.r#single_occurrence_date,
                ),
                to_pulumi_object_field(
                    "startTime",
                    &self.r#start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanBackupScheduleRpoConfigExclusionWindow {
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
                    r#daily: {
                        let field_value = match fields_map.get("daily") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_week: {
                        let field_value = match fields_map.get("daysOfWeek") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysOfWeek' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_occurrence_date: {
                        let field_value = match fields_map.get("singleOccurrenceDate") {
                            Some(value) => value,
                            None => bail!("Missing field 'singleOccurrenceDate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
