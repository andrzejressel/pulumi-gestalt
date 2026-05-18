#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResourcePolicySnapshotSchedulePolicySchedule {
    /// The policy will execute every nth day at the specified time.
    #[builder(into)]
    #[serde(rename = "dailySchedules")]
    pub r#daily_schedules: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyScheduleDailySchedule>,
    /// The policy will execute every nth hour starting at the specified time.
    #[builder(into)]
    #[serde(rename = "hourlySchedules")]
    pub r#hourly_schedules: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyScheduleHourlySchedule>,
    /// Allows specifying a snapshot time for each day of the week.
    #[builder(into)]
    #[serde(rename = "weeklySchedules")]
    pub r#weekly_schedules: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyScheduleWeeklySchedule>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResourcePolicySnapshotSchedulePolicySchedule {
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
                    "daily_schedules",
                    &self.r#daily_schedules,
                ),
                to_pulumi_object_field(
                    "hourly_schedules",
                    &self.r#hourly_schedules,
                ),
                to_pulumi_object_field(
                    "weekly_schedules",
                    &self.r#weekly_schedules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResourcePolicySnapshotSchedulePolicySchedule {
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
                    r#daily_schedules: {
                        let field_value = match fields_map.get("daily_schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily_schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hourly_schedules: {
                        let field_value = match fields_map.get("hourly_schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'hourly_schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_schedules: {
                        let field_value = match fields_map.get("weekly_schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly_schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
