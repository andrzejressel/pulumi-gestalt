#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecification {
    /// Customized capacity metric specification. The field is only valid when you use `customized_load_metric_specification`
    #[builder(into)]
    #[serde(rename = "customizedCapacityMetricSpecification")]
    pub r#customized_capacity_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecification>>,
    /// Customized load metric specification.
    #[builder(into)]
    #[serde(rename = "customizedLoadMetricSpecification")]
    pub r#customized_load_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecification>>,
    /// Customized scaling metric specification.
    #[builder(into)]
    #[serde(rename = "customizedScalingMetricSpecification")]
    pub r#customized_scaling_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecification>>,
    /// Predefined load metric specification.
    #[builder(into)]
    #[serde(rename = "predefinedLoadMetricSpecification")]
    pub r#predefined_load_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedLoadMetricSpecification>>,
    /// Metric pair specification from which Amazon EC2 Auto Scaling determines the appropriate scaling metric and load metric to use.
    #[builder(into)]
    #[serde(rename = "predefinedMetricPairSpecification")]
    pub r#predefined_metric_pair_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedMetricPairSpecification>>,
    /// Predefined scaling metric specification.
    #[builder(into)]
    #[serde(rename = "predefinedScalingMetricSpecification")]
    pub r#predefined_scaling_metric_specification: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedScalingMetricSpecification>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyPredictiveScalingConfigurationMetricSpecification {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("customized_capacity_metric_specification".to_string(), self.r#customized_capacity_metric_specification.to_pulumi_value().await);
            map.insert("customized_load_metric_specification".to_string(), self.r#customized_load_metric_specification.to_pulumi_value().await);
            map.insert("customized_scaling_metric_specification".to_string(), self.r#customized_scaling_metric_specification.to_pulumi_value().await);
            map.insert("predefined_load_metric_specification".to_string(), self.r#predefined_load_metric_specification.to_pulumi_value().await);
            map.insert("predefined_metric_pair_specification".to_string(), self.r#predefined_metric_pair_specification.to_pulumi_value().await);
            map.insert("predefined_scaling_metric_specification".to_string(), self.r#predefined_scaling_metric_specification.to_pulumi_value().await);
            map.insert("target_value".to_string(), self.r#target_value.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyPredictiveScalingConfigurationMetricSpecification {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#customized_capacity_metric_specification: {
                        let field_value = match fields_map.get("customized_capacity_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_capacity_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#customized_load_metric_specification: {
                        let field_value = match fields_map.get("customized_load_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_load_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#customized_scaling_metric_specification: {
                        let field_value = match fields_map.get("customized_scaling_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_scaling_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predefined_load_metric_specification: {
                        let field_value = match fields_map.get("predefined_load_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_load_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedLoadMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predefined_metric_pair_specification: {
                        let field_value = match fields_map.get("predefined_metric_pair_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_metric_pair_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedMetricPairSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predefined_scaling_metric_specification: {
                        let field_value = match fields_map.get("predefined_scaling_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_scaling_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedScalingMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_value: {
                        let field_value = match fields_map.get("target_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
