#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationBackupRetentionPolicy {
    /// A `daily_schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicyDailySchedule>>,
    /// The retention policy type of the backup policy. Possible value is `LongTermRetentionPolicy`. Defaults to `LongTermRetentionPolicy`.
    #[builder(into)]
    #[serde(rename = "retentionPolicyType")]
    pub r#retention_policy_type: Option<String>,
    /// A `weekly_schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicyWeeklySchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationBackupRetentionPolicy {
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
                "retention_policy_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_policy_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationBackupRetentionPolicy {
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
                    r#retention_policy_type: {
                        let field_value = match fields_map.get("retention_policy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_policy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
