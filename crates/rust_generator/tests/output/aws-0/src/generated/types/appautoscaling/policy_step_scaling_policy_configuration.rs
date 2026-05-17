#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyStepScalingPolicyConfiguration {
    /// Whether the adjustment is an absolute number or a percentage of the current capacity. Valid values are `ChangeInCapacity`, `ExactCapacity`, and `PercentChangeInCapacity`.
    #[builder(into)]
    #[serde(rename = "adjustmentType")]
    pub r#adjustment_type: Option<String>,
    /// Amount of time, in seconds, after a scaling activity completes and before the next scaling activity can start.
    #[builder(into)]
    #[serde(rename = "cooldown")]
    pub r#cooldown: Option<i32>,
    /// Aggregation type for the policy's metrics. Valid values are "Minimum", "Maximum", and "Average". Without a value, AWS will treat the aggregation type as "Average".
    #[builder(into)]
    #[serde(rename = "metricAggregationType")]
    pub r#metric_aggregation_type: Option<String>,
    /// Minimum number to adjust your scalable dimension as a result of a scaling activity. If the adjustment type is PercentChangeInCapacity, the scaling policy changes the scalable dimension of the scalable target by this amount.
    #[builder(into)]
    #[serde(rename = "minAdjustmentMagnitude")]
    pub r#min_adjustment_magnitude: Option<i32>,
    /// Set of adjustments that manage scaling. These have the following structure:
    /// 
    /// ```yaml
    /// resources:
    ///   ecsPolicy:
    ///     type: aws:appautoscaling:Policy
    ///     name: ecs_policy
    ///     properties:
    ///       stepScalingPolicyConfiguration:
    ///         stepAdjustments:
    ///           - metricIntervalLowerBound: 1
    ///             metricIntervalUpperBound: 2
    ///             scalingAdjustment: -1
    ///           - metricIntervalLowerBound: 2
    ///             metricIntervalUpperBound: 3
    ///             scalingAdjustment: 1
    /// ```
    #[builder(into)]
    #[serde(rename = "stepAdjustments")]
    pub r#step_adjustments: Option<Vec<super::super::types::appautoscaling::PolicyStepScalingPolicyConfigurationStepAdjustment>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyStepScalingPolicyConfiguration {
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
                "adjustment_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adjustment_type,
                )
                .await,
            );
            map.insert(
                "cooldown".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cooldown,
                )
                .await,
            );
            map.insert(
                "metric_aggregation_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_aggregation_type,
                )
                .await,
            );
            map.insert(
                "min_adjustment_magnitude".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_adjustment_magnitude,
                )
                .await,
            );
            map.insert(
                "step_adjustments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#step_adjustments,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyStepScalingPolicyConfiguration {
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
                    r#adjustment_type: {
                        let field_value = match fields_map.get("adjustment_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'adjustment_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cooldown: {
                        let field_value = match fields_map.get("cooldown") {
                            Some(value) => value,
                            None => bail!("Missing field 'cooldown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_aggregation_type: {
                        let field_value = match fields_map.get("metric_aggregation_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_aggregation_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_adjustment_magnitude: {
                        let field_value = match fields_map.get("min_adjustment_magnitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_adjustment_magnitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#step_adjustments: {
                        let field_value = match fields_map.get("step_adjustments") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_adjustments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
