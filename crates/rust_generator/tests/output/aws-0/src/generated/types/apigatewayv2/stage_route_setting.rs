#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StageRouteSetting {
    /// Whether data trace logging is enabled for the route. Affects the log entries pushed to Amazon CloudWatch Logs.
    /// Defaults to `false`. Supported only for WebSocket APIs.
    #[builder(into)]
    #[serde(rename = "dataTraceEnabled")]
    pub r#data_trace_enabled: Option<bool>,
    /// Whether detailed metrics are enabled for the route. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "detailedMetricsEnabled")]
    pub r#detailed_metrics_enabled: Option<bool>,
    /// Logging level for the route. Affects the log entries pushed to Amazon CloudWatch Logs.
    /// Valid values: `ERROR`, `INFO`, `OFF`. Defaults to `OFF`. Supported only for WebSocket APIs. This provider will only perform drift detection of its value when present in a configuration.
    #[builder(into)]
    #[serde(rename = "loggingLevel")]
    pub r#logging_level: Option<String>,
    /// Route key.
    #[builder(into)]
    #[serde(rename = "routeKey")]
    pub r#route_key: String,
    /// Throttling burst limit for the route.
    #[builder(into)]
    #[serde(rename = "throttlingBurstLimit")]
    pub r#throttling_burst_limit: Option<i32>,
    /// Throttling rate limit for the route.
    #[builder(into)]
    #[serde(rename = "throttlingRateLimit")]
    pub r#throttling_rate_limit: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StageRouteSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_trace_enabled".to_string(), self.r#data_trace_enabled.to_pulumi_value().await);
            map.insert("detailed_metrics_enabled".to_string(), self.r#detailed_metrics_enabled.to_pulumi_value().await);
            map.insert("logging_level".to_string(), self.r#logging_level.to_pulumi_value().await);
            map.insert("route_key".to_string(), self.r#route_key.to_pulumi_value().await);
            map.insert("throttling_burst_limit".to_string(), self.r#throttling_burst_limit.to_pulumi_value().await);
            map.insert("throttling_rate_limit".to_string(), self.r#throttling_rate_limit.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StageRouteSetting {
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
                    r#data_trace_enabled: {
                        let field_value = match fields_map.get("data_trace_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_trace_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#detailed_metrics_enabled: {
                        let field_value = match fields_map.get("detailed_metrics_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_metrics_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#logging_level: {
                        let field_value = match fields_map.get("logging_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#route_key: {
                        let field_value = match fields_map.get("route_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#throttling_burst_limit: {
                        let field_value = match fields_map.get("throttling_burst_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttling_burst_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#throttling_rate_limit: {
                        let field_value = match fields_map.get("throttling_rate_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttling_rate_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
