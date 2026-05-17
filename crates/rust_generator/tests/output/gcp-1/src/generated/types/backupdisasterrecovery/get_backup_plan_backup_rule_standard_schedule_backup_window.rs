#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackupPlanBackupRuleStandardScheduleBackupWindow {
    /// The hour of the day (1-24) when the window ends, for example, if the value of end hour of the day is 10, that means the backup window end time is 10:00.
    /// The end hour of the day should be greater than the start
    #[builder(into)]
    #[serde(rename = "endHourOfDay")]
    pub r#end_hour_of_day: i32,
    /// The hour of the day (0-23) when the window starts, for example, if the value of the start hour of the day is 6, that means the backup window starts at 6:00.
    #[builder(into)]
    #[serde(rename = "startHourOfDay")]
    pub r#start_hour_of_day: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackupPlanBackupRuleStandardScheduleBackupWindow {
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
                "end_hour_of_day".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_hour_of_day,
                )
                .await,
            );
            map.insert(
                "start_hour_of_day".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_hour_of_day,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackupPlanBackupRuleStandardScheduleBackupWindow {
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
                    r#end_hour_of_day: {
                        let field_value = match fields_map.get("end_hour_of_day") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_hour_of_day' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_hour_of_day: {
                        let field_value = match fields_map.get("start_hour_of_day") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_hour_of_day' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
