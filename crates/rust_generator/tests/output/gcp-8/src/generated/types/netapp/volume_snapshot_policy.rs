#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeSnapshotPolicy {
    /// Daily schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyDailySchedule>>,
    /// Enables automated snapshot creation according to defined schedule. Default is false.
    /// To disable automatic snapshot creation you have to remove the whole snapshot_policy block.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Hourly schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hourlySchedule")]
    pub r#hourly_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyHourlySchedule>>,
    /// Monthly schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "monthlySchedule")]
    pub r#monthly_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyMonthlySchedule>>,
    /// Weekly schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyWeeklySchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeSnapshotPolicy {
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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
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
                "monthly_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monthly_schedule,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeSnapshotPolicy {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#monthly_schedule: {
                        let field_value = match fields_map.get("monthly_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
