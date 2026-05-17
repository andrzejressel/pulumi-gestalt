#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SoftwareUpdateConfigurationSchedule {
    /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
    #[builder(into)]
    #[serde(rename = "advancedMonthDays")]
    pub r#advanced_month_days: Option<Vec<i32>>,
    /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values include `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`.
    #[builder(into)]
    #[serde(rename = "advancedWeekDays")]
    pub r#advanced_week_days: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Option<String>,
    /// A description for this Schedule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The end time of the schedule.
    #[builder(into)]
    #[serde(rename = "expiryTime")]
    pub r#expiry_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "expiryTimeOffsetMinutes")]
    pub r#expiry_time_offset_minutes: Option<f64>,
    /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
    /// Whether the schedule is enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "isEnabled")]
    pub r#is_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "lastModifiedTime")]
    pub r#last_modified_time: Option<String>,
    /// List of `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields as defined below.
    #[builder(into)]
    #[serde(rename = "monthlyOccurrence")]
    pub r#monthly_occurrence: Option<Box<super::super::types::automation::SoftwareUpdateConfigurationScheduleMonthlyOccurrence>>,
    #[builder(into)]
    #[serde(rename = "nextRun")]
    pub r#next_run: Option<String>,
    #[builder(into)]
    #[serde(rename = "nextRunOffsetMinutes")]
    pub r#next_run_offset_minutes: Option<f64>,
    /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "startTimeOffsetMinutes")]
    pub r#start_time_offset_minutes: Option<f64>,
    /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SoftwareUpdateConfigurationSchedule {
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
                "advanced_month_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_month_days,
                )
                .await,
            );
            map.insert(
                "advanced_week_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_week_days,
                )
                .await,
            );
            map.insert(
                "creation_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#creation_time,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "expiry_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiry_time,
                )
                .await,
            );
            map.insert(
                "expiry_time_offset_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiry_time_offset_minutes,
                )
                .await,
            );
            map.insert(
                "frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frequency,
                )
                .await,
            );
            map.insert(
                "interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interval,
                )
                .await,
            );
            map.insert(
                "is_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_enabled,
                )
                .await,
            );
            map.insert(
                "last_modified_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_modified_time,
                )
                .await,
            );
            map.insert(
                "monthly_occurrence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monthly_occurrence,
                )
                .await,
            );
            map.insert(
                "next_run".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_run,
                )
                .await,
            );
            map.insert(
                "next_run_offset_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_run_offset_minutes,
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
            map.insert(
                "start_time_offset_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time_offset_minutes,
                )
                .await,
            );
            map.insert(
                "time_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_zone,
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
                        let field_value = match fields_map.get("advanced_month_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_month_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_week_days: {
                        let field_value = match fields_map.get("advanced_week_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_week_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creation_time: {
                        let field_value = match fields_map.get("creation_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'creation_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("expiry_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiry_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiry_time_offset_minutes: {
                        let field_value = match fields_map.get("expiry_time_offset_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiry_time_offset_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("is_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_modified_time: {
                        let field_value = match fields_map.get("last_modified_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_modified_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_occurrence: {
                        let field_value = match fields_map.get("monthly_occurrence") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_occurrence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_run: {
                        let field_value = match fields_map.get("next_run") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_run' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_run_offset_minutes: {
                        let field_value = match fields_map.get("next_run_offset_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_run_offset_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#start_time_offset_minutes: {
                        let field_value = match fields_map.get("start_time_offset_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time_offset_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
