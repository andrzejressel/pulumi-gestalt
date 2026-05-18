#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetGroupHealthCheck {
    /// Whether health checks are enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Number of consecutive health check successes required before considering a target healthy. The range is 2-10. Defaults to 3.
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Option<i32>,
    /// Approximate amount of time, in seconds, between health checks of an individual target. The range is 5-300. For `lambda` target groups, it needs to be greater than the timeout of the underlying `lambda`. Defaults to 30.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
    /// The HTTP or gRPC codes to use when checking for a successful response from a target.
    /// The `health_check.protocol` must be one of `HTTP` or `HTTPS` or the `target_type` must be `lambda`.
    /// Values can be comma-separated individual values (e.g., "200,202") or a range of values (e.g., "200-299").
    /// * For gRPC-based target groups (i.e., the `protocol` is one of `HTTP` or `HTTPS` and the `protocol_version` is `GRPC`), values can be between `0` and `99`. The default is `12`.
    /// * When used with an Application Load Balancer (i.e., the `protocol` is one of `HTTP` or `HTTPS` and the `protocol_version` is not `GRPC`), values can be between `200` and `499`. The default is `200`.
    /// * When used with a Network Load Balancer (i.e., the `protocol` is one of `TCP`, `TCP_UDP`, `UDP`, or `TLS`), values can be between `200` and `599`. The default is `200-399`.
    /// * When the `target_type` is `lambda`, values can be between `200` and `499`. The default is `200`.
    #[builder(into)]
    #[serde(rename = "matcher")]
    pub r#matcher: Option<String>,
    /// Destination for the health check request. Required for HTTP/HTTPS ALB and HTTP NLB. Only applies to HTTP/HTTPS.
    /// * For HTTP and HTTPS health checks, the default is `/`.
    /// * For gRPC health checks, the default is `/AWS.ALB/healthcheck`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The port the load balancer uses when performing health checks on targets.
    /// Valid values are either `traffic-port`, to use the same port as the target group, or a valid port number between `1` and `65536`.
    /// Default is `traffic-port`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// Protocol the load balancer uses when performing health checks on targets.
    /// Must be one of `TCP`, `HTTP`, or `HTTPS`.
    /// The `TCP` protocol is not supported for health checks if the protocol of the target group is `HTTP` or `HTTPS`.
    /// Default is `HTTP`.
    /// Cannot be specified when the `target_type` is `lambda`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Amount of time, in seconds, during which no response from a target means a failed health check. The range is 2–120 seconds. For target groups with a protocol of HTTP, the default is 6 seconds. For target groups with a protocol of TCP, TLS or HTTPS, the default is 10 seconds. For target groups with a protocol of GENEVE, the default is 5 seconds. If the target type is lambda, the default is 30 seconds.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<i32>,
    /// Number of consecutive health check failures required before considering a target unhealthy. The range is 2-10. Defaults to 3.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetGroupHealthCheck {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "healthy_threshold",
                    &self.r#healthy_threshold,
                ),
                to_pulumi_object_field(
                    "interval",
                    &self.r#interval,
                ),
                to_pulumi_object_field(
                    "matcher",
                    &self.r#matcher,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
                to_pulumi_object_field(
                    "unhealthy_threshold",
                    &self.r#unhealthy_threshold,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetGroupHealthCheck {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#healthy_threshold: {
                        let field_value = match fields_map.get("healthy_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthy_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#matcher: {
                        let field_value = match fields_map.get("matcher") {
                            Some(value) => value,
                            None => bail!("Missing field 'matcher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unhealthy_threshold: {
                        let field_value = match fields_map.get("unhealthy_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'unhealthy_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
