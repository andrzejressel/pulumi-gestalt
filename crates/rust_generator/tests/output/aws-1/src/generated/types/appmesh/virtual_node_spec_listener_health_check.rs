#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListenerHealthCheck {
    /// Number of consecutive successful health checks that must occur before declaring listener healthy.
    #[builder(into)]
    pub r#healthy_threshold: i32,
    /// Time period in milliseconds between each health check execution.
    #[builder(into)]
    pub r#interval_millis: i32,
    /// Destination path for the health check request. This is only required if the specified protocol is `http` or `http2`.
    #[builder(into)]
    pub r#path: Option<String>,
    /// Destination port for the health check request. This port must match the port defined in the `port_mapping` for the listener.
    #[builder(into)]
    pub r#port: Option<i32>,
    /// Protocol for the health check request. Valid values are `http`, `http2`, `tcp` and `grpc`.
    #[builder(into)]
    pub r#protocol: String,
    /// Amount of time to wait when receiving a response from the health check, in milliseconds.
    #[builder(into)]
    pub r#timeout_millis: i32,
    /// Number of consecutive failed health checks that must occur before declaring a virtual node unhealthy.
    #[builder(into)]
    pub r#unhealthy_threshold: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecListenerHealthCheck {
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
                    "healthyThreshold",
                    &self.r#healthy_threshold,
                ),
                to_pulumi_object_field(
                    "intervalMillis",
                    &self.r#interval_millis,
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
                    "timeoutMillis",
                    &self.r#timeout_millis,
                ),
                to_pulumi_object_field(
                    "unhealthyThreshold",
                    &self.r#unhealthy_threshold,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecListenerHealthCheck {
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
                    r#healthy_threshold: {
                        let field_value = match fields_map.get("healthyThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthyThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_millis: {
                        let field_value = match fields_map.get("intervalMillis") {
                            Some(value) => value,
                            None => bail!("Missing field 'intervalMillis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#timeout_millis: {
                        let field_value = match fields_map.get("timeoutMillis") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutMillis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unhealthy_threshold: {
                        let field_value = match fields_map.get("unhealthyThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'unhealthyThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
