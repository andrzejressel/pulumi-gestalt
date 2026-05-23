#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscalerAutoscalingPolicy {
    /// The number of seconds that the autoscaler should wait before it
    /// starts collecting information from a new instance. This prevents
    /// the autoscaler from collecting information when the instance is
    /// initializing, during which the collected usage would not be
    /// reliable. The default time autoscaler waits is 60 seconds.
    /// Virtual machine initialization times might vary because of
    /// numerous factors. We recommend that you test how long an
    /// instance may take to initialize. To do this, create an instance
    /// and time the startup process.
    #[builder(into)]
    pub r#cooldown_period: Option<i32>,
    /// Defines the CPU utilization policy that allows the autoscaler to
    /// scale based on the average CPU utilization of a managed instance
    /// group.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cpu_utilization: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyCpuUtilization>>,
    /// Configuration parameters of autoscaling based on a load balancer.
    /// Structure is documented below.
    #[builder(into)]
    pub r#load_balancing_utilization: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyLoadBalancingUtilization>>,
    /// The maximum number of instances that the autoscaler can scale up
    /// to. This is required when creating or updating an autoscaler. The
    /// maximum number of replicas should not be lower than minimal number
    /// of replicas.
    #[builder(into)]
    pub r#max_replicas: i32,
    /// Configuration parameters of autoscaling based on a custom metric.
    /// Structure is documented below.
    #[builder(into)]
    pub r#metrics: Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyMetric>>,
    /// The minimum number of replicas that the autoscaler can scale down
    /// to. This cannot be less than 0. If not provided, autoscaler will
    /// choose a default value depending on maximum number of instances
    /// allowed.
    #[builder(into)]
    pub r#min_replicas: i32,
    /// Defines operating mode for this policy.
    #[builder(into)]
    pub r#mode: Option<String>,
    /// Defines scale down controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    pub r#scale_down_control: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleDownControl>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    pub r#scale_in_control: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleInControl>>,
    /// Scaling schedules defined for an autoscaler. Multiple schedules can be set on an autoscaler and they can overlap.
    /// Structure is documented below.
    #[builder(into)]
    pub r#scaling_schedules: Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyScalingSchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscalerAutoscalingPolicy {
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
                    "cooldownPeriod",
                    &self.r#cooldown_period,
                ),
                to_pulumi_object_field(
                    "cpuUtilization",
                    &self.r#cpu_utilization,
                ),
                to_pulumi_object_field(
                    "loadBalancingUtilization",
                    &self.r#load_balancing_utilization,
                ),
                to_pulumi_object_field(
                    "maxReplicas",
                    &self.r#max_replicas,
                ),
                to_pulumi_object_field(
                    "metrics",
                    &self.r#metrics,
                ),
                to_pulumi_object_field(
                    "minReplicas",
                    &self.r#min_replicas,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "scaleDownControl",
                    &self.r#scale_down_control,
                ),
                to_pulumi_object_field(
                    "scaleInControl",
                    &self.r#scale_in_control,
                ),
                to_pulumi_object_field(
                    "scalingSchedules",
                    &self.r#scaling_schedules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscalerAutoscalingPolicy {
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
                    r#cooldown_period: {
                        let field_value = match fields_map.get("cooldownPeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'cooldownPeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_utilization: {
                        let field_value = match fields_map.get("cpuUtilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuUtilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancing_utilization: {
                        let field_value = match fields_map.get("loadBalancingUtilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancingUtilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_replicas: {
                        let field_value = match fields_map.get("maxReplicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxReplicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metrics: {
                        let field_value = match fields_map.get("metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_replicas: {
                        let field_value = match fields_map.get("minReplicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'minReplicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scale_down_control: {
                        let field_value = match fields_map.get("scaleDownControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_in_control: {
                        let field_value = match fields_map.get("scaleInControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleInControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scaling_schedules: {
                        let field_value = match fields_map.get("scalingSchedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'scalingSchedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
