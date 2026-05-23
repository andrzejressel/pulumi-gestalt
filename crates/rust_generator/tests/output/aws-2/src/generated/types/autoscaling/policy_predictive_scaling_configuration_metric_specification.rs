#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecification {
    /// Customized capacity metric specification. The field is only valid when you use `customized_load_metric_specification`
    #[builder(into)]
    pub r#customized_capacity_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecification>>,
    /// Customized load metric specification.
    #[builder(into)]
    pub r#customized_load_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecification>>,
    /// Customized scaling metric specification.
    #[builder(into)]
    pub r#customized_scaling_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecification>>,
    /// Predefined load metric specification.
    #[builder(into)]
    pub r#predefined_load_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedLoadMetricSpecification>>,
    /// Metric pair specification from which Amazon EC2 Auto Scaling determines the appropriate scaling metric and load metric to use.
    #[builder(into)]
    pub r#predefined_metric_pair_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedMetricPairSpecification>>,
    /// Predefined scaling metric specification.
    #[builder(into)]
    pub r#predefined_scaling_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedScalingMetricSpecification>>,
    /// Target value for the metric.
    #[builder(into)]
    pub r#target_value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyPredictiveScalingConfigurationMetricSpecification {
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
                    "customizedCapacityMetricSpecification",
                    &self.r#customized_capacity_metric_specification,
                ),
                to_pulumi_object_field(
                    "customizedLoadMetricSpecification",
                    &self.r#customized_load_metric_specification,
                ),
                to_pulumi_object_field(
                    "customizedScalingMetricSpecification",
                    &self.r#customized_scaling_metric_specification,
                ),
                to_pulumi_object_field(
                    "predefinedLoadMetricSpecification",
                    &self.r#predefined_load_metric_specification,
                ),
                to_pulumi_object_field(
                    "predefinedMetricPairSpecification",
                    &self.r#predefined_metric_pair_specification,
                ),
                to_pulumi_object_field(
                    "predefinedScalingMetricSpecification",
                    &self.r#predefined_scaling_metric_specification,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyPredictiveScalingConfigurationMetricSpecification {
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
                    r#customized_capacity_metric_specification: {
                        let field_value = match fields_map.get("customizedCapacityMetricSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customizedCapacityMetricSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customized_load_metric_specification: {
                        let field_value = match fields_map.get("customizedLoadMetricSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customizedLoadMetricSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customized_scaling_metric_specification: {
                        let field_value = match fields_map.get("customizedScalingMetricSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customizedScalingMetricSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predefined_load_metric_specification: {
                        let field_value = match fields_map.get("predefinedLoadMetricSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefinedLoadMetricSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predefined_metric_pair_specification: {
                        let field_value = match fields_map.get("predefinedMetricPairSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefinedMetricPairSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
