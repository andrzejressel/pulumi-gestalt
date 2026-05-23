#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetGroupConfigHealthCheck {
    /// Indicates whether health checking is enabled. Defaults to `true`.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// The approximate amount of time, in seconds, between health checks of an individual target. The range is 5–300 seconds. The default is 30 seconds.
    #[builder(into)]
    pub r#health_check_interval_seconds: Option<i32>,
    /// The amount of time, in seconds, to wait before reporting a target as unhealthy. The range is 1–120 seconds. The default is 5 seconds.
    /// * `healthy_threshold_count ` - (Optional) The number of consecutive successful health checks required before considering an unhealthy target healthy. The range is 2–10. The default is 5.
    #[builder(into)]
    pub r#health_check_timeout_seconds: Option<i32>,
    #[builder(into)]
    pub r#healthy_threshold_count: Option<i32>,
    /// The codes to use when checking for a successful response from a target. These are called _Success codes_ in the console.
    #[builder(into)]
    pub r#matcher: Option<Box<super::super::types::vpclattice::TargetGroupConfigHealthCheckMatcher>>,
    /// The destination for health checks on the targets. If the protocol version is HTTP/1.1 or HTTP/2, specify a valid URI (for example, /path?query). The default path is `/`. Health checks are not supported if the protocol version is gRPC, however, you can choose HTTP/1.1 or HTTP/2 and specify a valid URI.
    #[builder(into)]
    pub r#path: Option<String>,
    /// The port used when performing health checks on targets. The default setting is the port that a target receives traffic on.
    #[builder(into)]
    pub r#port: Option<i32>,
    /// The protocol used when performing health checks on targets. The possible protocols are `HTTP` and `HTTPS`.
    #[builder(into)]
    pub r#protocol: Option<String>,
    /// The protocol version used when performing health checks on targets. The possible protocol versions are `HTTP1` and `HTTP2`. The default is `HTTP1`.
    #[builder(into)]
    pub r#protocol_version: Option<String>,
    /// The number of consecutive failed health checks required before considering a target unhealthy. The range is 2–10. The default is 2.
    #[builder(into)]
    pub r#unhealthy_threshold_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetGroupConfigHealthCheck {
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
                    "healthCheckIntervalSeconds",
                    &self.r#health_check_interval_seconds,
                ),
                to_pulumi_object_field(
                    "healthCheckTimeoutSeconds",
                    &self.r#health_check_timeout_seconds,
                ),
                to_pulumi_object_field(
                    "healthyThresholdCount",
                    &self.r#healthy_threshold_count,
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
                    "protocolVersion",
                    &self.r#protocol_version,
                ),
                to_pulumi_object_field(
                    "unhealthyThresholdCount",
                    &self.r#unhealthy_threshold_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetGroupConfigHealthCheck {
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
                    r#health_check_interval_seconds: {
                        let field_value = match fields_map.get("healthCheckIntervalSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckIntervalSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_timeout_seconds: {
                        let field_value = match fields_map.get("healthCheckTimeoutSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckTimeoutSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#healthy_threshold_count: {
                        let field_value = match fields_map.get("healthyThresholdCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthyThresholdCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#protocol_version: {
                        let field_value = match fields_map.get("protocolVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocolVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unhealthy_threshold_count: {
                        let field_value = match fields_map.get("unhealthyThresholdCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'unhealthyThresholdCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
