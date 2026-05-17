#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutonomousDatabasePropertiesScheduledOperationDetail {
    /// Possible values:
    ///  DAY_OF_WEEK_UNSPECIFIED
    /// MONDAY
    /// TUESDAY
    /// WEDNESDAY
    /// THURSDAY
    /// FRIDAY
    /// SATURDAY
    /// SUNDAY
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Option<String>,
    /// Represents a time of day. The date and time zone are either not significant
    /// or are specified elsewhere. An API may choose to allow leap seconds. Related
    /// types are google.type.Date and 'google.protobuf.Timestamp'.
    #[builder(into)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesScheduledOperationDetailStartTime>>,
    /// Represents a time of day. The date and time zone are either not significant
    /// or are specified elsewhere. An API may choose to allow leap seconds. Related
    /// types are google.type.Date and 'google.protobuf.Timestamp'.
    #[builder(into)]
    #[serde(rename = "stopTimes")]
    pub r#stop_times: Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesScheduledOperationDetailStopTime>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutonomousDatabasePropertiesScheduledOperationDetail {
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
                "day_of_week".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#day_of_week,
                )
                .await,
            );
            map.insert(
                "start_times".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_times,
                )
                .await,
            );
            map.insert(
                "stop_times".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stop_times,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutonomousDatabasePropertiesScheduledOperationDetail {
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
                    r#day_of_week: {
                        let field_value = match fields_map.get("day_of_week") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_week' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_times: {
                        let field_value = match fields_map.get("start_times") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_times' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stop_times: {
                        let field_value = match fields_map.get("stop_times") {
                            Some(value) => value,
                            None => bail!("Missing field 'stop_times' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
