#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListenerHealthCheck {
    /// Number of consecutive successful health checks that must occur before declaring listener healthy.
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: i32,
    /// Time period in milliseconds between each health check execution.
    #[builder(into)]
    #[serde(rename = "intervalMillis")]
    pub r#interval_millis: i32,
    /// Destination path for the health check request. This is only required if the specified protocol is `http` or `http2`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Destination port for the health check request. This port must match the port defined in the `port_mapping` for the listener.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Protocol for the health check request. Valid values are `http`, `http2`, `tcp` and `grpc`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Amount of time to wait when receiving a response from the health check, in milliseconds.
    #[builder(into)]
    #[serde(rename = "timeoutMillis")]
    pub r#timeout_millis: i32,
    /// Number of consecutive failed health checks that must occur before declaring a virtual node unhealthy.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecListenerHealthCheck {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("healthy_threshold".to_string(), self.r#healthy_threshold.to_pulumi_value().await);
            map.insert("interval_millis".to_string(), self.r#interval_millis.to_pulumi_value().await);
            map.insert("path".to_string(), self.r#path.to_pulumi_value().await);
            map.insert("port".to_string(), self.r#port.to_pulumi_value().await);
            map.insert("protocol".to_string(), self.r#protocol.to_pulumi_value().await);
            map.insert("timeout_millis".to_string(), self.r#timeout_millis.to_pulumi_value().await);
            map.insert("unhealthy_threshold".to_string(), self.r#unhealthy_threshold.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecListenerHealthCheck {
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
                    r#healthy_threshold: {
                        let field_value = match fields_map.get("healthy_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthy_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#interval_millis: {
                        let field_value = match fields_map.get("interval_millis") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_millis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_millis: {
                        let field_value = match fields_map.get("timeout_millis") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_millis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#unhealthy_threshold: {
                        let field_value = match fields_map.get("unhealthy_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'unhealthy_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
