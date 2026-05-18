#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StageDefaultRouteSettings {
    /// Whether data trace logging is enabled for the default route. Affects the log entries pushed to Amazon CloudWatch Logs.
    /// Defaults to `false`. Supported only for WebSocket APIs.
    #[builder(into)]
    #[serde(rename = "dataTraceEnabled")]
    pub r#data_trace_enabled: Option<bool>,
    /// Whether detailed metrics are enabled for the default route. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "detailedMetricsEnabled")]
    pub r#detailed_metrics_enabled: Option<bool>,
    /// Logging level for the default route. Affects the log entries pushed to Amazon CloudWatch Logs.
    /// Valid values: `ERROR`, `INFO`, `OFF`. Defaults to `OFF`. Supported only for WebSocket APIs. This provider will only perform drift detection of its value when present in a configuration.
    #[builder(into)]
    #[serde(rename = "loggingLevel")]
    pub r#logging_level: Option<String>,
    /// Throttling burst limit for the default route.
    #[builder(into)]
    #[serde(rename = "throttlingBurstLimit")]
    pub r#throttling_burst_limit: Option<i32>,
    /// Throttling rate limit for the default route.
    #[builder(into)]
    #[serde(rename = "throttlingRateLimit")]
    pub r#throttling_rate_limit: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StageDefaultRouteSettings {
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
                    "data_trace_enabled",
                    &self.r#data_trace_enabled,
                ),
                to_pulumi_object_field(
                    "detailed_metrics_enabled",
                    &self.r#detailed_metrics_enabled,
                ),
                to_pulumi_object_field(
                    "logging_level",
                    &self.r#logging_level,
                ),
                to_pulumi_object_field(
                    "throttling_burst_limit",
                    &self.r#throttling_burst_limit,
                ),
                to_pulumi_object_field(
                    "throttling_rate_limit",
                    &self.r#throttling_rate_limit,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StageDefaultRouteSettings {
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
                    r#data_trace_enabled: {
                        let field_value = match fields_map.get("data_trace_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_trace_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_metrics_enabled: {
                        let field_value = match fields_map.get("detailed_metrics_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_metrics_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_level: {
                        let field_value = match fields_map.get("logging_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throttling_burst_limit: {
                        let field_value = match fields_map.get("throttling_burst_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttling_burst_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throttling_rate_limit: {
                        let field_value = match fields_map.get("throttling_rate_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttling_rate_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
