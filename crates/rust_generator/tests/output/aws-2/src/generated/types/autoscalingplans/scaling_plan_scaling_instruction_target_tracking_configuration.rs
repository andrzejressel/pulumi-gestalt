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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "customized_scaling_metric_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#customized_scaling_metric_specification,
                )
                .await,
            );
            map.insert(
                "disable_scale_in".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_scale_in,
                )
                .await,
            );
            map.insert(
                "estimated_instance_warmup".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#estimated_instance_warmup,
                )
                .await,
            );
            map.insert(
                "predefined_scaling_metric_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#predefined_scaling_metric_specification,
                )
                .await,
            );
            map.insert(
                "scale_in_cooldown".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale_in_cooldown,
                )
                .await,
            );
            map.insert(
                "scale_out_cooldown".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale_out_cooldown,
                )
                .await,
            );
            map.insert(
                "target_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_value,
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
                        let field_value = match fields_map.get("customized_scaling_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_scaling_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_scale_in: {
                        let field_value = match fields_map.get("disable_scale_in") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_scale_in' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#estimated_instance_warmup: {
                        let field_value = match fields_map.get("estimated_instance_warmup") {
                            Some(value) => value,
                            None => bail!("Missing field 'estimated_instance_warmup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predefined_scaling_metric_specification: {
                        let field_value = match fields_map.get("predefined_scaling_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_scaling_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_in_cooldown: {
                        let field_value = match fields_map.get("scale_in_cooldown") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_in_cooldown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_out_cooldown: {
                        let field_value = match fields_map.get("scale_out_cooldown") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_out_cooldown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_value: {
                        let field_value = match fields_map.get("target_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
