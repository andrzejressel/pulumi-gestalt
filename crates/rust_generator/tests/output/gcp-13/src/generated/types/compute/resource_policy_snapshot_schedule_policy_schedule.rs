#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourcePolicySnapshotSchedulePolicySchedule {
    /// The policy will execute every nth day at the specified time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Option<Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleDailySchedule>>,
    /// The policy will execute every nth hour starting at the specified time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hourlySchedule")]
    pub r#hourly_schedule: Option<Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleHourlySchedule>>,
    /// Allows specifying a snapshot time for each day of the week.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleWeeklySchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourcePolicySnapshotSchedulePolicySchedule {
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
                "daily_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#daily_schedule,
                )
                .await,
            );
            map.insert(
                "hourly_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hourly_schedule,
                )
                .await,
            );
            map.insert(
                "weekly_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekly_schedule,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourcePolicySnapshotSchedulePolicySchedule {
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
                    r#daily_schedule: {
                        let field_value = match fields_map.get("daily_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hourly_schedule: {
                        let field_value = match fields_map.get("hourly_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'hourly_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_schedule: {
                        let field_value = match fields_map.get("weekly_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
