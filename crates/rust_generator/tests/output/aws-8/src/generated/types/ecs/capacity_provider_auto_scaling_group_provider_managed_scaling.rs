#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CapacityProviderAutoScalingGroupProviderManagedScaling {
    /// Period of time, in seconds, after a newly launched Amazon EC2 instance can contribute to CloudWatch metrics for Auto Scaling group. If this parameter is omitted, the default value of 300 seconds is used.
    /// 
    /// For more information on how the instance warmup period contributes to managed scale-out behavior, see [Control the instances Amazon ECS terminates](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/managed-termination-protection.html) in the _Amazon Elastic Container Service Developer Guide_.
    #[builder(into)]
    pub r#instance_warmup_period: Option<i32>,
    /// Maximum step adjustment size. A number between 1 and 10,000.
    #[builder(into)]
    pub r#maximum_scaling_step_size: Option<i32>,
    /// Minimum step adjustment size. A number between 1 and 10,000.
    #[builder(into)]
    pub r#minimum_scaling_step_size: Option<i32>,
    /// Whether auto scaling is managed by ECS. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    pub r#status: Option<String>,
    /// Target utilization for the capacity provider. A number between 1 and 100.
    #[builder(into)]
    pub r#target_capacity: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CapacityProviderAutoScalingGroupProviderManagedScaling {
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
                    "instanceWarmupPeriod",
                    &self.r#instance_warmup_period,
                ),
                to_pulumi_object_field(
                    "maximumScalingStepSize",
                    &self.r#maximum_scaling_step_size,
                ),
                to_pulumi_object_field(
                    "minimumScalingStepSize",
                    &self.r#minimum_scaling_step_size,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "targetCapacity",
                    &self.r#target_capacity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CapacityProviderAutoScalingGroupProviderManagedScaling {
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
                    r#instance_warmup_period: {
                        let field_value = match fields_map.get("instanceWarmupPeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceWarmupPeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_scaling_step_size: {
                        let field_value = match fields_map.get("maximumScalingStepSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumScalingStepSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_scaling_step_size: {
                        let field_value = match fields_map.get("minimumScalingStepSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimumScalingStepSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_capacity: {
                        let field_value = match fields_map.get("targetCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
