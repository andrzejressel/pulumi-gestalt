#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionAutomaticScaling {
    /// The time period that the Autoscaler should wait before it starts collecting information from a new instance.
    /// This prevents the autoscaler from collecting information when the instance is initializing,
    /// during which the collected usage would not be reliable. Default: 120s
    #[builder(into)]
    #[serde(rename = "coolDownPeriod")]
    pub r#cool_down_period: Option<String>,
    /// Target scaling by CPU usage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cpuUtilization")]
    pub r#cpu_utilization: Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingCpuUtilization>,
    /// Target scaling by disk usage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "diskUtilization")]
    pub r#disk_utilization: Option<Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingDiskUtilization>>,
    /// Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.
    /// Defaults to a runtime-specific value.
    #[builder(into)]
    #[serde(rename = "maxConcurrentRequests")]
    pub r#max_concurrent_requests: Option<i32>,
    /// Maximum number of idle instances that should be maintained for this version.
    #[builder(into)]
    #[serde(rename = "maxIdleInstances")]
    pub r#max_idle_instances: Option<i32>,
    /// Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
    #[builder(into)]
    #[serde(rename = "maxPendingLatency")]
    pub r#max_pending_latency: Option<String>,
    /// Maximum number of instances that should be started to handle requests for this version. Default: 20
    #[builder(into)]
    #[serde(rename = "maxTotalInstances")]
    pub r#max_total_instances: Option<i32>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
    #[builder(into)]
    #[serde(rename = "minIdleInstances")]
    pub r#min_idle_instances: Option<i32>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    #[builder(into)]
    #[serde(rename = "minPendingLatency")]
    pub r#min_pending_latency: Option<String>,
    /// Minimum number of running instances that should be maintained for this version. Default: 2
    #[builder(into)]
    #[serde(rename = "minTotalInstances")]
    pub r#min_total_instances: Option<i32>,
    /// Target scaling by network usage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "networkUtilization")]
    pub r#network_utilization: Option<Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingNetworkUtilization>>,
    /// Target scaling by request utilization.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestUtilization")]
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
                    "cool_down_period",
                    &self.r#cool_down_period,
                ),
                to_pulumi_object_field(
                    "cpu_utilization",
                    &self.r#cpu_utilization,
                ),
                to_pulumi_object_field(
                    "disk_utilization",
                    &self.r#disk_utilization,
                ),
                to_pulumi_object_field(
                    "max_concurrent_requests",
                    &self.r#max_concurrent_requests,
                ),
                to_pulumi_object_field(
                    "max_idle_instances",
                    &self.r#max_idle_instances,
                ),
                to_pulumi_object_field(
                    "max_pending_latency",
                    &self.r#max_pending_latency,
                ),
                to_pulumi_object_field(
                    "max_total_instances",
                    &self.r#max_total_instances,
                ),
                to_pulumi_object_field(
                    "min_idle_instances",
                    &self.r#min_idle_instances,
                ),
                to_pulumi_object_field(
                    "min_pending_latency",
                    &self.r#min_pending_latency,
                ),
                to_pulumi_object_field(
                    "min_total_instances",
                    &self.r#min_total_instances,
                ),
                to_pulumi_object_field(
                    "network_utilization",
                    &self.r#network_utilization,
                ),
                to_pulumi_object_field(
                    "request_utilization",
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
                        let field_value = match fields_map.get("cool_down_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'cool_down_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#disk_utilization: {
                        let field_value = match fields_map.get("disk_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_requests: {
                        let field_value = match fields_map.get("max_concurrent_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_idle_instances: {
                        let field_value = match fields_map.get("max_idle_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_idle_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pending_latency: {
                        let field_value = match fields_map.get("max_pending_latency") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pending_latency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_total_instances: {
                        let field_value = match fields_map.get("max_total_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_total_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_idle_instances: {
                        let field_value = match fields_map.get("min_idle_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_idle_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_pending_latency: {
                        let field_value = match fields_map.get("min_pending_latency") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_pending_latency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_total_instances: {
                        let field_value = match fields_map.get("min_total_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_total_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_utilization: {
                        let field_value = match fields_map.get("network_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_utilization: {
                        let field_value = match fields_map.get("request_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
