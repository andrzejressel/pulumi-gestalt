#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule {
    /// A list containing a single item, which specifies the Hour interval at which this recurrence should be triggered.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<Vec<i32>>,
    /// A list containing a single item which specifies the Minute interval at which this recurrence should be triggered.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<Vec<i32>>,
    /// A list of days of the month that the job should execute on.
    #[builder(into)]
    #[serde(rename = "monthDays")]
    pub r#month_days: Option<Vec<i32>>,
    /// A `monthly` block as documented below.
    #[builder(into)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Option<Vec<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceScheduleMonthly>>,
    /// A list of days of the week that the job should execute on. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekDays")]
    pub r#week_days: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule {
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
                "month_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#month_days,
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
            map.insert(
                "week_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#week_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule {
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
                    r#month_days: {
                        let field_value = match fields_map.get("month_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'month_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#week_days: {
                        let field_value = match fields_map.get("week_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'week_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
