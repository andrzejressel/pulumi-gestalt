#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerScheduleSchedule {
    /// Day(s) of the month on which the trigger is scheduled. This value can be specified with a monthly frequency only.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Option<Vec<i32>>,
    /// Days of the week on which the trigger is scheduled. This value can be specified only with a weekly frequency.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Hours of the day on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<Vec<i32>>,
    /// Minutes of the hour on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<Vec<i32>>,
    /// A `monthly` block as documented below, which specifies the days of the month on which the trigger is scheduled. The value can be specified only with a monthly frequency.
    #[builder(into)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Option<Vec<super::super::types::datafactory::TriggerScheduleScheduleMonthly>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerScheduleSchedule {
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
                "days_of_months".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#days_of_months,
                )
                .await,
            );
            map.insert(
                "days_of_weeks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#days_of_weeks,
                )
                .await,
            );
            map.insert(
                "hours".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hours,
                )
                .await,
            );
            map.insert(
                "minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minutes,
                )
                .await,
            );
            map.insert(
                "monthlies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monthlies,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerScheduleSchedule {
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
