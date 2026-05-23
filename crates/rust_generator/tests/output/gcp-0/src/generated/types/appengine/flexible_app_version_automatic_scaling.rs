#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionAutomaticScaling {
    /// The time period that the Autoscaler should wait before it starts collecting information from a new instance.
    /// This prevents the autoscaler from collecting information when the instance is initializing,
    /// during which the collected usage would not be reliable. Default: 120s
    #[builder(into)]
    pub r#cool_down_period: Option<String>,
    /// Target scaling by CPU usage.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cpu_utilization: Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingCpuUtilization>,
    /// Target scaling by disk usage.
    /// Structure is documented below.
    #[builder(into)]
    pub r#disk_utilization: Option<Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingDiskUtilization>>,
    /// Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.
    /// Defaults to a runtime-specific value.
    #[builder(into)]
    pub r#max_concurrent_requests: Option<i32>,
    /// Maximum number of idle instances that should be maintained for this version.
    #[builder(into)]
    pub r#max_idle_instances: Option<i32>,
    /// Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
    #[builder(into)]
    pub r#max_pending_latency: Option<String>,
    /// Maximum number of instances that should be started to handle requests for this version. Default: 20
    #[builder(into)]
    pub r#max_total_instances: Option<i32>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
    #[builder(into)]
    pub r#min_idle_instances: Option<i32>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    #[builder(into)]
    pub r#min_pending_latency: Option<String>,
    /// Minimum number of running instances that should be maintained for this version. Default: 2
    #[builder(into)]
    pub r#min_total_instances: Option<i32>,
    /// Target scaling by network usage.
    /// Structure is documented below.
    #[builder(into)]
    pub r#network_utilization: Option<Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingNetworkUtilization>>,
    /// Target scaling by request utilization.
    /// Structure is documented below.
    #[builder(into)]
    pub r#request_utilization: Option<Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingRequestUtilization>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionAutomaticScaling {
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
                    "coolDownPeriod",
                    &self.r#cool_down_period,
                ),
                to_pulumi_object_field(
                    "cpuUtilization",
                    &self.r#cpu_utilization,
                ),
                to_pulumi_object_field(
                    "diskUtilization",
                    &self.r#disk_utilization,
                ),
                to_pulumi_object_field(
                    "maxConcurrentRequests",
                    &self.r#max_concurrent_requests,
                ),
                to_pulumi_object_field(
                    "maxIdleInstances",
                    &self.r#max_idle_instances,
                ),
                to_pulumi_object_field(
                    "maxPendingLatency",
                    &self.r#max_pending_latency,
                ),
                to_pulumi_object_field(
                    "maxTotalInstances",
                    &self.r#max_total_instances,
                ),
                to_pulumi_object_field(
                    "minIdleInstances",
                    &self.r#min_idle_instances,
                ),
                to_pulumi_object_field(
                    "minPendingLatency",
                    &self.r#min_pending_latency,
                ),
                to_pulumi_object_field(
                    "minTotalInstances",
                    &self.r#min_total_instances,
                ),
                to_pulumi_object_field(
                    "networkUtilization",
                    &self.r#network_utilization,
                ),
                to_pulumi_object_field(
                    "requestUtilization",
                    &self.r#request_utilization,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionAutomaticScaling {
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
                    r#cool_down_period: {
                        let field_value = match fields_map.get("coolDownPeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'coolDownPeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#disk_utilization: {
                        let field_value = match fields_map.get("diskUtilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskUtilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_requests: {
                        let field_value = match fields_map.get("maxConcurrentRequests") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConcurrentRequests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_idle_instances: {
                        let field_value = match fields_map.get("maxIdleInstances") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxIdleInstances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pending_latency: {
                        let field_value = match fields_map.get("maxPendingLatency") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxPendingLatency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_total_instances: {
                        let field_value = match fields_map.get("maxTotalInstances") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxTotalInstances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_idle_instances: {
                        let field_value = match fields_map.get("minIdleInstances") {
                            Some(value) => value,
                            None => bail!("Missing field 'minIdleInstances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_pending_latency: {
                        let field_value = match fields_map.get("minPendingLatency") {
                            Some(value) => value,
                            None => bail!("Missing field 'minPendingLatency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_total_instances: {
                        let field_value = match fields_map.get("minTotalInstances") {
                            Some(value) => value,
                            None => bail!("Missing field 'minTotalInstances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_utilization: {
                        let field_value = match fields_map.get("networkUtilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkUtilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_utilization: {
                        let field_value = match fields_map.get("requestUtilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestUtilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
