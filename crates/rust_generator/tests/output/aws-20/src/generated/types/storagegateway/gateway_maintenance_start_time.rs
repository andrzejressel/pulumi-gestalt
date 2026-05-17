#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewayMaintenanceStartTime {
    /// The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Option<String>,
    /// The day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Option<String>,
    /// The hour component of the maintenance start time represented as _hh_, where _hh_ is the hour (00 to 23). The hour of the day is in the time zone of the gateway.
    #[builder(into)]
    #[serde(rename = "hourOfDay")]
    pub r#hour_of_day: i32,
    /// The minute component of the maintenance start time represented as _mm_, where _mm_ is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.
    #[builder(into)]
    #[serde(rename = "minuteOfHour")]
    pub r#minute_of_hour: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GatewayMaintenanceStartTime {
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
                "day_of_month".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#day_of_month,
                )
                .await,
            );
            map.insert(
                "day_of_week".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#day_of_week,
                )
                .await,
            );
            map.insert(
                "hour_of_day".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hour_of_day,
                )
                .await,
            );
            map.insert(
                "minute_of_hour".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minute_of_hour,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GatewayMaintenanceStartTime {
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
                    r#day_of_month: {
                        let field_value = match fields_map.get("day_of_month") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_month' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#day_of_week: {
                        let field_value = match fields_map.get("day_of_week") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_week' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hour_of_day: {
                        let field_value = match fields_map.get("hour_of_day") {
                            Some(value) => value,
                            None => bail!("Missing field 'hour_of_day' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minute_of_hour: {
                        let field_value = match fields_map.get("minute_of_hour") {
                            Some(value) => value,
                            None => bail!("Missing field 'minute_of_hour' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
