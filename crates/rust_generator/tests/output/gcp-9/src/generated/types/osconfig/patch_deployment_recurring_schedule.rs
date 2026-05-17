#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentRecurringSchedule {
    /// The end time at which a recurring patch deployment schedule is no longer active.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// (Output)
    /// The time the last patch job ran successfully.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "lastExecuteTime")]
    pub r#last_execute_time: Option<String>,
    /// Schedule with monthly executions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "monthly")]
    pub r#monthly: Option<Box<super::super::types::osconfig::PatchDeploymentRecurringScheduleMonthly>>,
    /// (Output)
    /// The time the next patch job is scheduled to run.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "nextExecuteTime")]
    pub r#next_execute_time: Option<String>,
    /// The time that the recurring schedule becomes effective. Defaults to createTime of the patch deployment.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Time of the day to run a recurring deployment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeOfDay")]
    pub r#time_of_day: Box<super::super::types::osconfig::PatchDeploymentRecurringScheduleTimeOfDay>,
    /// Defines the time zone that timeOfDay is relative to. The rules for daylight saving time are
    /// determined by the chosen time zone.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<super::super::types::osconfig::PatchDeploymentRecurringScheduleTimeZone>,
    /// Schedule with weekly executions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weekly")]
    pub r#weekly: Option<Box<super::super::types::osconfig::PatchDeploymentRecurringScheduleWeekly>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentRecurringSchedule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "end_time",
                    &self.r#end_time,
                ),
                to_pulumi_object_field(
                    "last_execute_time",
                    &self.r#last_execute_time,
                ),
                to_pulumi_object_field(
                    "monthly",
                    &self.r#monthly,
                ),
                to_pulumi_object_field(
                    "next_execute_time",
                    &self.r#next_execute_time,
                ),
                to_pulumi_object_field(
                    "start_time",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "time_of_day",
                    &self.r#time_of_day,
                ),
                to_pulumi_object_field(
                    "time_zone",
                    &self.r#time_zone,
                ),
                to_pulumi_object_field(
                    "weekly",
                    &self.r#weekly,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentRecurringSchedule {
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
                    r#end_time: {
                        let field_value = match fields_map.get("end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_execute_time: {
                        let field_value = match fields_map.get("last_execute_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_execute_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly: {
                        let field_value = match fields_map.get("monthly") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_execute_time: {
                        let field_value = match fields_map.get("next_execute_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_execute_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#time_of_day: {
                        let field_value = match fields_map.get("time_of_day") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_of_day' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#weekly: {
                        let field_value = match fields_map.get("weekly") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
