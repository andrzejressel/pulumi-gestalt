#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScalingPlanScalingInstruction {
    /// Customized load metric to use for predictive scaling. You must specify either `customized_load_metric_specification` or `predefined_load_metric_specification` when configuring predictive scaling.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_CustomizedLoadMetricSpecification.html).
    #[builder(into)]
    #[serde(rename = "customizedLoadMetricSpecification")]
    pub r#customized_load_metric_specification: Option<Box<super::super::types::autoscalingplans::ScalingPlanScalingInstructionCustomizedLoadMetricSpecification>>,
    /// Boolean controlling whether dynamic scaling by AWS Auto Scaling is disabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "disableDynamicScaling")]
    pub r#disable_dynamic_scaling: Option<bool>,
    /// Maximum capacity of the resource. The exception to this upper limit is if you specify a non-default setting for `predictive_scaling_max_capacity_behavior`.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: i32,
    /// Minimum capacity of the resource.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: i32,
    /// Predefined load metric to use for predictive scaling. You must specify either `predefined_load_metric_specification` or `customized_load_metric_specification` when configuring predictive scaling.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_PredefinedLoadMetricSpecification.html).
    #[builder(into)]
    #[serde(rename = "predefinedLoadMetricSpecification")]
    pub r#predefined_load_metric_specification: Option<Box<super::super::types::autoscalingplans::ScalingPlanScalingInstructionPredefinedLoadMetricSpecification>>,
    /// Defines the behavior that should be applied if the forecast capacity approaches or exceeds the maximum capacity specified for the resource.
    /// Valid values: `SetForecastCapacityToMaxCapacity`, `SetMaxCapacityAboveForecastCapacity`, `SetMaxCapacityToForecastCapacity`.
    #[builder(into)]
    #[serde(rename = "predictiveScalingMaxCapacityBehavior")]
    pub r#predictive_scaling_max_capacity_behavior: Option<String>,
    /// Size of the capacity buffer to use when the forecast capacity is close to or exceeds the maximum capacity.
    #[builder(into)]
    #[serde(rename = "predictiveScalingMaxCapacityBuffer")]
    pub r#predictive_scaling_max_capacity_buffer: Option<i32>,
    /// Predictive scaling mode. Valid values: `ForecastAndScale`, `ForecastOnly`.
    #[builder(into)]
    #[serde(rename = "predictiveScalingMode")]
    pub r#predictive_scaling_mode: Option<String>,
    /// ID of the resource. This string consists of the resource type and unique identifier.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: String,
    /// Scalable dimension associated with the resource. Valid values: `autoscaling:autoScalingGroup:DesiredCapacity`, `dynamodb:index:ReadCapacityUnits`, `dynamodb:index:WriteCapacityUnits`, `dynamodb:table:ReadCapacityUnits`, `dynamodb:table:WriteCapacityUnits`, `ecs:service:DesiredCount`, `ec2:spot-fleet-request:TargetCapacity`, `rds:cluster:ReadReplicaCount`.
    #[builder(into)]
    #[serde(rename = "scalableDimension")]
    pub r#scalable_dimension: String,
    /// Controls whether a resource's externally created scaling policies are kept or replaced. Valid values: `KeepExternalPolicies`, `ReplaceExternalPolicies`. Defaults to `KeepExternalPolicies`.
    #[builder(into)]
    #[serde(rename = "scalingPolicyUpdateBehavior")]
    pub r#scaling_policy_update_behavior: Option<String>,
    /// Amount of time, in seconds, to buffer the run time of scheduled scaling actions when scaling out.
    #[builder(into)]
    #[serde(rename = "scheduledActionBufferTime")]
    pub r#scheduled_action_buffer_time: Option<i32>,
    /// Namespace of the AWS service. Valid values: `autoscaling`, `dynamodb`, `ecs`, `ec2`, `rds`.
    #[builder(into)]
    #[serde(rename = "serviceNamespace")]
    pub r#service_namespace: String,
    /// Structure that defines new target tracking configurations. Each of these structures includes a specific scaling metric and a target value for the metric, along with various parameters to use with dynamic scaling.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_TargetTrackingConfiguration.html).
    #[builder(into)]
    #[serde(rename = "targetTrackingConfigurations")]
    pub r#target_tracking_configurations: Vec<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScalingPlanScalingInstruction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("customized_load_metric_specification".to_string(), self.r#customized_load_metric_specification.to_pulumi_value().await);
            map.insert("disable_dynamic_scaling".to_string(), self.r#disable_dynamic_scaling.to_pulumi_value().await);
            map.insert("max_capacity".to_string(), self.r#max_capacity.to_pulumi_value().await);
            map.insert("min_capacity".to_string(), self.r#min_capacity.to_pulumi_value().await);
            map.insert("predefined_load_metric_specification".to_string(), self.r#predefined_load_metric_specification.to_pulumi_value().await);
            map.insert("predictive_scaling_max_capacity_behavior".to_string(), self.r#predictive_scaling_max_capacity_behavior.to_pulumi_value().await);
            map.insert("predictive_scaling_max_capacity_buffer".to_string(), self.r#predictive_scaling_max_capacity_buffer.to_pulumi_value().await);
            map.insert("predictive_scaling_mode".to_string(), self.r#predictive_scaling_mode.to_pulumi_value().await);
            map.insert("resource_id".to_string(), self.r#resource_id.to_pulumi_value().await);
            map.insert("scalable_dimension".to_string(), self.r#scalable_dimension.to_pulumi_value().await);
            map.insert("scaling_policy_update_behavior".to_string(), self.r#scaling_policy_update_behavior.to_pulumi_value().await);
            map.insert("scheduled_action_buffer_time".to_string(), self.r#scheduled_action_buffer_time.to_pulumi_value().await);
            map.insert("service_namespace".to_string(), self.r#service_namespace.to_pulumi_value().await);
            map.insert("target_tracking_configurations".to_string(), self.r#target_tracking_configurations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScalingPlanScalingInstruction {
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
                    r#customized_load_metric_specification: {
                        let field_value = match fields_map.get("customized_load_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_load_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscalingplans::ScalingPlanScalingInstructionCustomizedLoadMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#disable_dynamic_scaling: {
                        let field_value = match fields_map.get("disable_dynamic_scaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_dynamic_scaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_capacity: {
                        let field_value = match fields_map.get("max_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_capacity: {
                        let field_value = match fields_map.get("min_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predefined_load_metric_specification: {
                        let field_value = match fields_map.get("predefined_load_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_load_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscalingplans::ScalingPlanScalingInstructionPredefinedLoadMetricSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predictive_scaling_max_capacity_behavior: {
                        let field_value = match fields_map.get("predictive_scaling_max_capacity_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'predictive_scaling_max_capacity_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predictive_scaling_max_capacity_buffer: {
                        let field_value = match fields_map.get("predictive_scaling_max_capacity_buffer") {
                            Some(value) => value,
                            None => bail!("Missing field 'predictive_scaling_max_capacity_buffer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#predictive_scaling_mode: {
                        let field_value = match fields_map.get("predictive_scaling_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'predictive_scaling_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_id: {
                        let field_value = match fields_map.get("resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scalable_dimension: {
                        let field_value = match fields_map.get("scalable_dimension") {
                            Some(value) => value,
                            None => bail!("Missing field 'scalable_dimension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scaling_policy_update_behavior: {
                        let field_value = match fields_map.get("scaling_policy_update_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaling_policy_update_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scheduled_action_buffer_time: {
                        let field_value = match fields_map.get("scheduled_action_buffer_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduled_action_buffer_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_namespace: {
                        let field_value = match fields_map.get("service_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_tracking_configurations: {
                        let field_value = match fields_map.get("target_tracking_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_tracking_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
