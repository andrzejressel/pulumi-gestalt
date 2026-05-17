#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyVmBackup {
    /// Sets the backup frequency. Possible values are `Hourly`, `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// Duration of the backup window in hours. Possible values are between `4` and `24` This is used when `frequency` is `Hourly`.
    /// 
    /// > **NOTE:** `hour_duration` must be multiplier of `hour_interval`
    #[builder(into)]
    #[serde(rename = "hourDuration")]
    pub r#hour_duration: Option<i32>,
    /// Interval in hour at which backup is triggered. Possible values are `4`, `6`, `8` and `12`. This is used when `frequency` is `Hourly`.
    #[builder(into)]
    #[serde(rename = "hourInterval")]
    pub r#hour_interval: Option<i32>,
    /// The time of day to perform the backup in 24hour format.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: String,
    /// The days of the week to perform backups on. Must be one of `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`. This is used when `frequency` is `Weekly`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyVmBackup {
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
                "frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frequency,
                )
                .await,
            );
            map.insert(
                "hour_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hour_duration,
                )
                .await,
            );
            map.insert(
                "hour_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hour_interval,
                )
                .await,
            );
            map.insert(
                "time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time,
                )
                .await,
            );
            map.insert(
                "weekdays".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekdays,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyVmBackup {
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
                    r#frequency: {
                        let field_value = match fields_map.get("frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hour_duration: {
                        let field_value = match fields_map.get("hour_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'hour_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hour_interval: {
                        let field_value = match fields_map.get("hour_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'hour_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time: {
                        let field_value = match fields_map.get("time") {
                            Some(value) => value,
                            None => bail!("Missing field 'time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekdays: {
                        let field_value = match fields_map.get("weekdays") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekdays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
