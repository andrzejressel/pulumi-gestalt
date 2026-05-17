#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IoTHubCloudToDeviceFeedback {
    /// The lock duration for the feedback queue, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 5 and 300 seconds. Defaults to `PT60S`.
    #[builder(into)]
    #[serde(rename = "lockDuration")]
    pub r#lock_duration: Option<String>,
    /// The maximum delivery count for the feedback queue. This value must be between `1` and `100`. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "maxDeliveryCount")]
    pub r#max_delivery_count: Option<i32>,
    /// The retention time for service-bound feedback messages, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 1 minute and 48 hours. Defaults to `PT1H`.
    #[builder(into)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IoTHubCloudToDeviceFeedback {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "lock_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lock_duration,
                )
                .await,
            );
            map.insert(
                "max_delivery_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_delivery_count,
                )
                .await,
            );
            map.insert(
                "time_to_live".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_to_live,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IoTHubCloudToDeviceFeedback {
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
                    r#lock_duration: {
                        let field_value = match fields_map.get("lock_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'lock_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_delivery_count: {
                        let field_value = match fields_map.get("max_delivery_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_delivery_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_to_live: {
                        let field_value = match fields_map.get("time_to_live") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_to_live' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
