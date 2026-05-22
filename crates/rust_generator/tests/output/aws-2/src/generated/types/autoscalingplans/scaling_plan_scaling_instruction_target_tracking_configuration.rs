#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScalingPlanScalingInstructionTargetTrackingConfiguration {
    /// Customized metric. You can specify either `customized_scaling_metric_specification` or `predefined_scaling_metric_specification`.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_CustomizedScalingMetricSpecification.html).
    #[builder(into)]
    #[serde(rename = "customizedScalingMetricSpecification")]
    pub r#customized_scaling_metric_specification: Option<Box<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfigurationCustomizedScalingMetricSpecification>>,
    /// Boolean indicating whether scale in by the target tracking scaling policy is disabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Option<bool>,
    /// Estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics.
    /// This value is used only if the resource is an Auto Scaling group.
    #[builder(into)]
    #[serde(rename = "estimatedInstanceWarmup")]
    pub r#estimated_instance_warmup: Option<i32>,
    /// Predefined metric. You can specify either `predefined_scaling_metric_specification` or `customized_scaling_metric_specification`.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_PredefinedScalingMetricSpecification.html).
    #[builder(into)]
    #[serde(rename = "predefinedScalingMetricSpecification")]
    pub r#predefined_scaling_metric_specification: Option<Box<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfigurationPredefinedScalingMetricSpecification>>,
    /// Amount of time, in seconds, after a scale in activity completes before another scale in activity can start.
    /// This value is not used if the scalable resource is an Auto Scaling group.
    #[builder(into)]
    #[serde(rename = "scaleInCooldown")]
    pub r#scale_in_cooldown: Option<i32>,
    /// Amount of time, in seconds, after a scale-out activity completes before another scale-out activity can start.
    /// This value is not used if the scalable resource is an Auto Scaling group.
    #[builder(into)]
    #[serde(rename = "scaleOutCooldown")]
    pub r#scale_out_cooldown: Option<i32>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScalingPlanScalingInstructionTargetTrackingConfiguration {
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
                    "customizedScalingMetricSpecification",
                    &self.r#customized_scaling_metric_specification,
                ),
                to_pulumi_object_field(
                    "disableScaleIn",
                    &self.r#disable_scale_in,
                ),
                to_pulumi_object_field(
                    "estimatedInstanceWarmup",
                    &self.r#estimated_instance_warmup,
                ),
                to_pulumi_object_field(
                    "predefinedScalingMetricSpecification",
                    &self.r#predefined_scaling_metric_specification,
                ),
                to_pulumi_object_field(
                    "scaleInCooldown",
                    &self.r#scale_in_cooldown,
                ),
                to_pulumi_object_field(
                    "scaleOutCooldown",
                    &self.r#scale_out_cooldown,
                ),
                to_pulumi_object_field(
                    "targetValue",
                    &self.r#target_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScalingPlanScalingInstructionTargetTrackingConfiguration {
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
                    r#customized_scaling_metric_specification: {
                        let field_value = match fields_map.get("customizedScalingMetricSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customizedScalingMetricSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_scale_in: {
                        let field_value = match fields_map.get("disableScaleIn") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableScaleIn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#estimated_instance_warmup: {
                        let field_value = match fields_map.get("estimatedInstanceWarmup") {
                            Some(value) => value,
                            None => bail!("Missing field 'estimatedInstanceWarmup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predefined_scaling_metric_specification: {
                        let field_value = match fields_map.get("predefinedScalingMetricSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefinedScalingMetricSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_in_cooldown: {
                        let field_value = match fields_map.get("scaleInCooldown") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleInCooldown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_out_cooldown: {
                        let field_value = match fields_map.get("scaleOutCooldown") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleOutCooldown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_value: {
                        let field_value = match fields_map.get("targetValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
