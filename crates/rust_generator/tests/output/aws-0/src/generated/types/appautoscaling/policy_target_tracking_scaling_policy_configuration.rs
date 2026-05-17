#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyTargetTrackingScalingPolicyConfiguration {
    /// Custom CloudWatch metric. Documentation can be found  at: [AWS Customized Metric Specification](https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_CustomizedMetricSpecification.html). See supported fields below.
    #[builder(into)]
    #[serde(rename = "customizedMetricSpecification")]
    pub r#customized_metric_specification: Option<Box<super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecification>>,
    /// Whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Option<bool>,
    /// Predefined metric. See supported fields below.
    #[builder(into)]
    #[serde(rename = "predefinedMetricSpecification")]
    pub r#predefined_metric_specification: Option<Box<super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification>>,
    /// Amount of time, in seconds, after a scale in activity completes before another scale in activity can start.
    #[builder(into)]
    #[serde(rename = "scaleInCooldown")]
    pub r#scale_in_cooldown: Option<i32>,
    /// Amount of time, in seconds, after a scale out activity completes before another scale out activity can start.
    #[builder(into)]
    #[serde(rename = "scaleOutCooldown")]
    pub r#scale_out_cooldown: Option<i32>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyTargetTrackingScalingPolicyConfiguration {
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
                "customized_metric_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#customized_metric_specification,
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
                "predefined_metric_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#predefined_metric_specification,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyTargetTrackingScalingPolicyConfiguration {
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
                    r#customized_metric_specification: {
                        let field_value = match fields_map.get("customized_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#predefined_metric_specification: {
                        let field_value = match fields_map.get("predefined_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
