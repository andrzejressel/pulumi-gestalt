#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContactsRotationRecurrence {
    #[builder(into)]
    pub r#daily_settings: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceDailySetting>>,
    /// (Optional) Information about on-call rotations that recur monthly. See Monthly Settings for more details.
    #[builder(into)]
    pub r#monthly_settings: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceMonthlySetting>>,
    /// (Required) The number of contacts, or shift team members designated to be on call concurrently during a shift.
    #[builder(into)]
    pub r#number_of_on_calls: i32,
    /// (Required) The number of days, weeks, or months a single rotation lasts.
    #[builder(into)]
    pub r#recurrence_multiplier: i32,
    /// (Optional) Information about the days of the week that the on-call rotation coverage includes. See Shift Coverages for more details.
    #[builder(into)]
    pub r#shift_coverages: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverage>>,
    /// (Optional) Information about on-call rotations that recur weekly. See Weekly Settings for more details.
    #[builder(into)]
    pub r#weekly_settings: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceWeeklySetting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContactsRotationRecurrence {
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
                    "dailySettings",
                    &self.r#daily_settings,
                ),
                to_pulumi_object_field(
                    "monthlySettings",
                    &self.r#monthly_settings,
                ),
                to_pulumi_object_field(
                    "numberOfOnCalls",
                    &self.r#number_of_on_calls,
                ),
                to_pulumi_object_field(
                    "recurrenceMultiplier",
                    &self.r#recurrence_multiplier,
                ),
                to_pulumi_object_field(
                    "shiftCoverages",
                    &self.r#shift_coverages,
                ),
                to_pulumi_object_field(
                    "weeklySettings",
                    &self.r#weekly_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContactsRotationRecurrence {
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
                    r#daily_settings: {
                        let field_value = match fields_map.get("dailySettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dailySettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_settings: {
                        let field_value = match fields_map.get("monthlySettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlySettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_on_calls: {
                        let field_value = match fields_map.get("numberOfOnCalls") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberOfOnCalls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recurrence_multiplier: {
                        let field_value = match fields_map.get("recurrenceMultiplier") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurrenceMultiplier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shift_coverages: {
                        let field_value = match fields_map.get("shiftCoverages") {
                            Some(value) => value,
                            None => bail!("Missing field 'shiftCoverages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_settings: {
                        let field_value = match fields_map.get("weeklySettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeklySettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
