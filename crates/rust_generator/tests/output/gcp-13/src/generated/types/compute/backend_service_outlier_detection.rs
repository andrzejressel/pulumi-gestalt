#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendServiceOutlierDetection {
    /// The base time that a host is ejected for. The real time is equal to the base
    /// time multiplied by the number of times the host has been ejected. Defaults to
    /// 30000ms or 30s.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "baseEjectionTime")]
    pub r#base_ejection_time: Option<Box<super::super::types::compute::BackendServiceOutlierDetectionBaseEjectionTime>>,
    /// Number of errors before a host is ejected from the connection pool. When the
    /// backend host is accessed over HTTP, a 5xx return code qualifies as an error.
    /// Defaults to 5.
    #[builder(into)]
    #[serde(rename = "consecutiveErrors")]
    pub r#consecutive_errors: Option<i32>,
    /// The number of consecutive gateway failures (502, 503, 504 status or connection
    /// errors that are mapped to one of those status codes) before a consecutive
    /// gateway failure ejection occurs. Defaults to 5.
    #[builder(into)]
    #[serde(rename = "consecutiveGatewayFailure")]
    pub r#consecutive_gateway_failure: Option<i32>,
    /// The percentage chance that a host will be actually ejected when an outlier
    /// status is detected through consecutive 5xx. This setting can be used to disable
    /// ejection or to ramp it up slowly. Defaults to 100.
    #[builder(into)]
    #[serde(rename = "enforcingConsecutiveErrors")]
    pub r#enforcing_consecutive_errors: Option<i32>,
    /// The percentage chance that a host will be actually ejected when an outlier
    /// status is detected through consecutive gateway failures. This setting can be
    /// used to disable ejection or to ramp it up slowly. Defaults to 0.
    #[builder(into)]
    #[serde(rename = "enforcingConsecutiveGatewayFailure")]
    pub r#enforcing_consecutive_gateway_failure: Option<i32>,
    /// The percentage chance that a host will be actually ejected when an outlier
    /// status is detected through success rate statistics. This setting can be used to
    /// disable ejection or to ramp it up slowly. Defaults to 100.
    #[builder(into)]
    #[serde(rename = "enforcingSuccessRate")]
    pub r#enforcing_success_rate: Option<i32>,
    /// Time interval between ejection sweep analysis. This can result in both new
    /// ejections as well as hosts being returned to service. Defaults to 10 seconds.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<Box<super::super::types::compute::BackendServiceOutlierDetectionInterval>>,
    /// Maximum percentage of hosts in the load balancing pool for the backend service
    /// that can be ejected. Defaults to 10%.
    #[builder(into)]
    #[serde(rename = "maxEjectionPercent")]
    pub r#max_ejection_percent: Option<i32>,
    /// The number of hosts in a cluster that must have enough request volume to detect
    /// success rate outliers. If the number of hosts is less than this setting, outlier
    /// detection via success rate statistics is not performed for any host in the
    /// cluster. Defaults to 5.
    #[builder(into)]
    #[serde(rename = "successRateMinimumHosts")]
    pub r#success_rate_minimum_hosts: Option<i32>,
    /// The minimum number of total requests that must be collected in one interval (as
    /// defined by the interval duration above) to include this host in success rate
    /// based outlier detection. If the volume is lower than this setting, outlier
    /// detection via success rate statistics is not performed for that host. Defaults
    /// to 100.
    #[builder(into)]
    #[serde(rename = "successRateRequestVolume")]
    pub r#success_rate_request_volume: Option<i32>,
    /// This factor is used to determine the ejection threshold for success rate outlier
    /// ejection. The ejection threshold is the difference between the mean success
    /// rate, and the product of this factor and the standard deviation of the mean
    /// success rate: mean - (stdev * success_rate_stdev_factor). This factor is divided
    /// by a thousand to get a double. That is, if the desired factor is 1.9, the
    /// runtime value should be 1900. Defaults to 1900.
    #[builder(into)]
    #[serde(rename = "successRateStdevFactor")]
    pub r#success_rate_stdev_factor: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendServiceOutlierDetection {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "base_ejection_time",
                    &self.r#base_ejection_time,
                ),
                to_pulumi_object_field(
                    "consecutive_errors",
                    &self.r#consecutive_errors,
                ),
                to_pulumi_object_field(
                    "consecutive_gateway_failure",
                    &self.r#consecutive_gateway_failure,
                ),
                to_pulumi_object_field(
                    "enforcing_consecutive_errors",
                    &self.r#enforcing_consecutive_errors,
                ),
                to_pulumi_object_field(
                    "enforcing_consecutive_gateway_failure",
                    &self.r#enforcing_consecutive_gateway_failure,
                ),
                to_pulumi_object_field(
                    "enforcing_success_rate",
                    &self.r#enforcing_success_rate,
                ),
                to_pulumi_object_field(
                    "interval",
                    &self.r#interval,
                ),
                to_pulumi_object_field(
                    "max_ejection_percent",
                    &self.r#max_ejection_percent,
                ),
                to_pulumi_object_field(
                    "success_rate_minimum_hosts",
                    &self.r#success_rate_minimum_hosts,
                ),
                to_pulumi_object_field(
                    "success_rate_request_volume",
                    &self.r#success_rate_request_volume,
                ),
                to_pulumi_object_field(
                    "success_rate_stdev_factor",
                    &self.r#success_rate_stdev_factor,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendServiceOutlierDetection {
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
                    r#base_ejection_time: {
                        let field_value = match fields_map.get("base_ejection_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_ejection_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consecutive_errors: {
                        let field_value = match fields_map.get("consecutive_errors") {
                            Some(value) => value,
                            None => bail!("Missing field 'consecutive_errors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consecutive_gateway_failure: {
                        let field_value = match fields_map.get("consecutive_gateway_failure") {
                            Some(value) => value,
                            None => bail!("Missing field 'consecutive_gateway_failure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforcing_consecutive_errors: {
                        let field_value = match fields_map.get("enforcing_consecutive_errors") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforcing_consecutive_errors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforcing_consecutive_gateway_failure: {
                        let field_value = match fields_map.get("enforcing_consecutive_gateway_failure") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforcing_consecutive_gateway_failure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforcing_success_rate: {
                        let field_value = match fields_map.get("enforcing_success_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforcing_success_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval: {
                        let field_value = match fields_map.get("interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ejection_percent: {
                        let field_value = match fields_map.get("max_ejection_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_ejection_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#success_rate_minimum_hosts: {
                        let field_value = match fields_map.get("success_rate_minimum_hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_rate_minimum_hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#success_rate_request_volume: {
                        let field_value = match fields_map.get("success_rate_request_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_rate_request_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#success_rate_stdev_factor: {
                        let field_value = match fields_map.get("success_rate_stdev_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_rate_stdev_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
