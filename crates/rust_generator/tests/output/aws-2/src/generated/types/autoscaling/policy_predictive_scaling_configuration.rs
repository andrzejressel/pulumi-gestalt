#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPredictiveScalingConfiguration {
    /// Defines the behavior that should be applied if the forecast capacity approaches or exceeds the maximum capacity of the Auto Scaling group. Valid values are `HonorMaxCapacity` or `IncreaseMaxCapacity`. Default is `HonorMaxCapacity`.
    #[builder(into)]
    #[serde(rename = "maxCapacityBreachBehavior")]
    pub r#max_capacity_breach_behavior: Option<String>,
    /// Size of the capacity buffer to use when the forecast capacity is close to or exceeds the maximum capacity. Valid range is `0` to `100`. If set to `0`, Amazon EC2 Auto Scaling may scale capacity higher than the maximum capacity to equal but not exceed forecast capacity.
    #[builder(into)]
    #[serde(rename = "maxCapacityBuffer")]
    pub r#max_capacity_buffer: Option<String>,
    /// This structure includes the metrics and target utilization to use for predictive scaling.
    #[builder(into)]
    #[serde(rename = "metricSpecification")]
    pub r#metric_specification: Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecification>,
    /// Predictive scaling mode. Valid values are `ForecastAndScale` and `ForecastOnly`. Default is `ForecastOnly`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Amount of time, in seconds, by which the instance launch time can be advanced. Minimum is `0`.
    #[builder(into)]
    #[serde(rename = "schedulingBufferTime")]
    pub r#scheduling_buffer_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyPredictiveScalingConfiguration {
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
                "max_capacity_breach_behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_capacity_breach_behavior,
                )
                .await,
            );
            map.insert(
                "max_capacity_buffer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_capacity_buffer,
                )
                .await,
            );
            map.insert(
                "metric_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_specification,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "scheduling_buffer_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scheduling_buffer_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyPredictiveScalingConfiguration {
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
                    r#max_capacity_breach_behavior: {
                        let field_value = match fields_map.get("max_capacity_breach_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_capacity_breach_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_capacity_buffer: {
                        let field_value = match fields_map.get("max_capacity_buffer") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_capacity_buffer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_specification: {
                        let field_value = match fields_map.get("metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduling_buffer_time: {
                        let field_value = match fields_map.get("scheduling_buffer_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduling_buffer_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
