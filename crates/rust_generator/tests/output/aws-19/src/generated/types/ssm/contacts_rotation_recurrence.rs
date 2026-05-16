#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContactsRotationRecurrence {
    #[builder(into)]
    #[serde(rename = "dailySettings")]
    pub r#daily_settings: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceDailySetting>>,
    /// (Optional) Information about on-call rotations that recur monthly. See Monthly Settings for more details.
    #[builder(into)]
    #[serde(rename = "monthlySettings")]
    pub r#monthly_settings: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceMonthlySetting>>,
    /// (Required) The number of contacts, or shift team members designated to be on call concurrently during a shift.
    #[builder(into)]
    #[serde(rename = "numberOfOnCalls")]
    pub r#number_of_on_calls: i32,
    /// (Required) The number of days, weeks, or months a single rotation lasts.
    #[builder(into)]
    #[serde(rename = "recurrenceMultiplier")]
    pub r#recurrence_multiplier: i32,
    /// (Optional) Information about the days of the week that the on-call rotation coverage includes. See Shift Coverages for more details.
    #[builder(into)]
    #[serde(rename = "shiftCoverages")]
    pub r#shift_coverages: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverage>>,
    /// (Optional) Information about on-call rotations that recur weekly. See Weekly Settings for more details.
    #[builder(into)]
    #[serde(rename = "weeklySettings")]
    pub r#weekly_settings: Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceWeeklySetting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContactsRotationRecurrence {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("daily_settings".to_string(), self.r#daily_settings.to_pulumi_value().await);
            map.insert("monthly_settings".to_string(), self.r#monthly_settings.to_pulumi_value().await);
            map.insert("number_of_on_calls".to_string(), self.r#number_of_on_calls.to_pulumi_value().await);
            map.insert("recurrence_multiplier".to_string(), self.r#recurrence_multiplier.to_pulumi_value().await);
            map.insert("shift_coverages".to_string(), self.r#shift_coverages.to_pulumi_value().await);
            map.insert("weekly_settings".to_string(), self.r#weekly_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContactsRotationRecurrence {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#daily_settings: {
                        let field_value = match fields_map.get("daily_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceDailySetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#monthly_settings: {
                        let field_value = match fields_map.get("monthly_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceMonthlySetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#number_of_on_calls: {
                        let field_value = match fields_map.get("number_of_on_calls") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_on_calls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#recurrence_multiplier: {
                        let field_value = match fields_map.get("recurrence_multiplier") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurrence_multiplier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#shift_coverages: {
                        let field_value = match fields_map.get("shift_coverages") {
                            Some(value) => value,
                            None => bail!("Missing field 'shift_coverages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverage>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#weekly_settings: {
                        let field_value = match fields_map.get("weekly_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceWeeklySetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
