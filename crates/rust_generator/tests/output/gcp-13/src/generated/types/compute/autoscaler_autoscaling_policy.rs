#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
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
    #[serde(rename = "cooldownPeriod")]
    pub r#cooldown_period: Option<i32>,
    /// Defines the CPU utilization policy that allows the autoscaler to
    /// scale based on the average CPU utilization of a managed instance
    /// group.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cpuUtilization")]
    pub r#cpu_utilization: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyCpuUtilization>>,
    /// Configuration parameters of autoscaling based on a load balancer.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "loadBalancingUtilization")]
    pub r#load_balancing_utilization: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyLoadBalancingUtilization>>,
    /// The maximum number of instances that the autoscaler can scale up
    /// to. This is required when creating or updating an autoscaler. The
    /// maximum number of replicas should not be lower than minimal number
    /// of replicas.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: i32,
    /// Configuration parameters of autoscaling based on a custom metric.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyMetric>>,
    /// The minimum number of replicas that the autoscaler can scale down
    /// to. This cannot be less than 0. If not provided, autoscaler will
    /// choose a default value depending on maximum number of instances
    /// allowed.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: i32,
    /// Defines operating mode for this policy.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Defines scale down controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scaleDownControl")]
    pub r#scale_down_control: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleDownControl>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scaleInControl")]
    pub r#scale_in_control: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleInControl>>,
    /// Scaling schedules defined for an autoscaler. Multiple schedules can be set on an autoscaler and they can overlap.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scalingSchedules")]
    pub r#scaling_schedules: Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyScalingSchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscalerAutoscalingPolicy {
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
                "cooldown_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cooldown_period,
                )
                .await,
            );
            map.insert(
                "cpu_utilization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_utilization,
                )
                .await,
            );
            map.insert(
                "load_balancing_utilization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancing_utilization,
                )
                .await,
            );
            map.insert(
                "max_replicas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_replicas,
                )
                .await,
            );
            map.insert(
                "metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metrics,
                )
                .await,
            );
            map.insert(
                "min_replicas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_replicas,
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
                "scale_down_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale_down_control,
                )
                .await,
            );
            map.insert(
                "scale_in_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale_in_control,
                )
                .await,
            );
            map.insert(
                "scaling_schedules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scaling_schedules,
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
                        let field_value = match fields_map.get("cooldown_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'cooldown_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_utilization: {
                        let field_value = match fields_map.get("cpu_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancing_utilization: {
                        let field_value = match fields_map.get("load_balancing_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancing_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_replicas: {
                        let field_value = match fields_map.get("max_replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("min_replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("scale_down_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_down_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_in_control: {
                        let field_value = match fields_map.get("scale_in_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_in_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scaling_schedules: {
                        let field_value = match fields_map.get("scaling_schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaling_schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
