#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerScheduleSchedule {
    /// Day(s) of the month on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Vec<i32>,
    /// Day(s) of the week on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Vec<String>,
    /// Hours of the day on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Vec<i32>,
    /// Minutes of the hour on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Vec<i32>,
    /// A `monthly` block as documented below, which specifies the days of the month on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Vec<super::super::types::datafactory::GetTriggerScheduleScheduleMonthly>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerScheduleSchedule {
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
                    "days_of_months",
                    &self.r#days_of_months,
                ),
                to_pulumi_object_field(
                    "days_of_weeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "hours",
                    &self.r#hours,
                ),
                to_pulumi_object_field(
                    "minutes",
                    &self.r#minutes,
                ),
                to_pulumi_object_field(
                    "monthlies",
                    &self.r#monthlies,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerScheduleSchedule {
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
                    r#hours: {
                        let field_value = match fields_map.get("hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minutes: {
                        let field_value = match fields_map.get("minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthlies: {
                        let field_value = match fields_map.get("monthlies") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
