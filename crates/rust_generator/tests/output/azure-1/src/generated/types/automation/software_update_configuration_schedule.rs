#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SoftwareUpdateConfigurationSchedule {
    /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
    #[builder(into)]
    pub r#advanced_month_days: Option<Vec<i32>>,
    /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values include `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`.
    #[builder(into)]
    pub r#advanced_week_days: Option<Vec<String>>,
    #[builder(into)]
    pub r#creation_time: Option<String>,
    /// A description for this Schedule.
    #[builder(into)]
    pub r#description: Option<String>,
    /// The end time of the schedule.
    #[builder(into)]
    pub r#expiry_time: Option<String>,
    #[builder(into)]
    pub r#expiry_time_offset_minutes: Option<f64>,
    /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
    #[builder(into)]
    pub r#frequency: String,
    /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month`.
    #[builder(into)]
    pub r#interval: Option<i32>,
    /// Whether the schedule is enabled. Defaults to `true`.
    #[builder(into)]
    pub r#is_enabled: Option<bool>,
    #[builder(into)]
    pub r#last_modified_time: Option<String>,
    /// List of `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields as defined below.
    #[builder(into)]
    pub r#monthly_occurrence: Option<Box<super::super::types::automation::SoftwareUpdateConfigurationScheduleMonthlyOccurrence>>,
    #[builder(into)]
    pub r#next_run: Option<String>,
    #[builder(into)]
    pub r#next_run_offset_minutes: Option<f64>,
    /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
    #[builder(into)]
    pub r#start_time: Option<String>,
    #[builder(into)]
    pub r#start_time_offset_minutes: Option<f64>,
    /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
    #[builder(into)]
    pub r#time_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SoftwareUpdateConfigurationSchedule {
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
                    "advancedMonthDays",
                    &self.r#advanced_month_days,
                ),
                to_pulumi_object_field(
                    "advancedWeekDays",
                    &self.r#advanced_week_days,
                ),
                to_pulumi_object_field(
                    "creationTime",
                    &self.r#creation_time,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "expiryTime",
                    &self.r#expiry_time,
                ),
                to_pulumi_object_field(
                    "expiryTimeOffsetMinutes",
                    &self.r#expiry_time_offset_minutes,
                ),
                to_pulumi_object_field(
                    "frequency",
                    &self.r#frequency,
                ),
                to_pulumi_object_field(
                    "interval",
                    &self.r#interval,
                ),
                to_pulumi_object_field(
                    "isEnabled",
                    &self.r#is_enabled,
                ),
                to_pulumi_object_field(
                    "lastModifiedTime",
                    &self.r#last_modified_time,
                ),
                to_pulumi_object_field(
                    "monthlyOccurrence",
                    &self.r#monthly_occurrence,
                ),
                to_pulumi_object_field(
                    "nextRun",
                    &self.r#next_run,
                ),
                to_pulumi_object_field(
                    "nextRunOffsetMinutes",
                    &self.r#next_run_offset_minutes,
                ),
                to_pulumi_object_field(
                    "startTime",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "startTimeOffsetMinutes",
                    &self.r#start_time_offset_minutes,
                ),
                to_pulumi_object_field(
                    "timeZone",
                    &self.r#time_zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SoftwareUpdateConfigurationSchedule {
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
                    r#advanced_month_days: {
                        let field_value = match fields_map.get("advancedMonthDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedMonthDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_week_days: {
                        let field_value = match fields_map.get("advancedWeekDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedWeekDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creation_time: {
                        let field_value = match fields_map.get("creationTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'creationTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiry_time: {
                        let field_value = match fields_map.get("expiryTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiryTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiry_time_offset_minutes: {
                        let field_value = match fields_map.get("expiryTimeOffsetMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiryTimeOffsetMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frequency: {
                        let field_value = match fields_map.get("frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval: {
                        let field_value = match fields_map.get("interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_enabled: {
                        let field_value = match fields_map.get("isEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'isEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_modified_time: {
                        let field_value = match fields_map.get("lastModifiedTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'lastModifiedTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_occurrence: {
                        let field_value = match fields_map.get("monthlyOccurrence") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlyOccurrence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_run: {
                        let field_value = match fields_map.get("nextRun") {
                            Some(value) => value,
                            None => bail!("Missing field 'nextRun' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_run_offset_minutes: {
                        let field_value = match fields_map.get("nextRunOffsetMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'nextRunOffsetMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#start_time_offset_minutes: {
                        let field_value = match fields_map.get("startTimeOffsetMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'startTimeOffsetMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
