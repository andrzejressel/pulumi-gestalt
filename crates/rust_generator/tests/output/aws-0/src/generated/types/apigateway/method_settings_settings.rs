#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MethodSettingsSettings {
    /// Whether the cached responses are encrypted.
    #[builder(into)]
    pub r#cache_data_encrypted: Option<bool>,
    /// Time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached.
    #[builder(into)]
    pub r#cache_ttl_in_seconds: Option<i32>,
    /// Whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached.
    #[builder(into)]
    pub r#caching_enabled: Option<bool>,
    /// Whether data trace logging is enabled for this method, which effects the log entries pushed to Amazon CloudWatch Logs.
    #[builder(into)]
    pub r#data_trace_enabled: Option<bool>,
    /// Logging level for this method, which effects the log entries pushed to Amazon CloudWatch Logs. The available levels are `OFF`, `ERROR`, and `INFO`.
    #[builder(into)]
    pub r#logging_level: Option<String>,
    /// Whether Amazon CloudWatch metrics are enabled for this method.
    #[builder(into)]
    pub r#metrics_enabled: Option<bool>,
    /// Whether authorization is required for a cache invalidation request.
    #[builder(into)]
    pub r#require_authorization_for_cache_control: Option<bool>,
    /// Throttling burst limit. Default: `-1` (throttling disabled).
    #[builder(into)]
    pub r#throttling_burst_limit: Option<i32>,
    /// Throttling rate limit. Default: `-1` (throttling disabled).
    #[builder(into)]
    pub r#throttling_rate_limit: Option<f64>,
    /// How to handle unauthorized requests for cache invalidation. The available values are `FAIL_WITH_403`, `SUCCEED_WITH_RESPONSE_HEADER`, `SUCCEED_WITHOUT_RESPONSE_HEADER`.
    #[builder(into)]
    pub r#unauthorized_cache_control_header_strategy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MethodSettingsSettings {
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
                    "cacheDataEncrypted",
                    &self.r#cache_data_encrypted,
                ),
                to_pulumi_object_field(
                    "cacheTtlInSeconds",
                    &self.r#cache_ttl_in_seconds,
                ),
                to_pulumi_object_field(
                    "cachingEnabled",
                    &self.r#caching_enabled,
                ),
                to_pulumi_object_field(
                    "dataTraceEnabled",
                    &self.r#data_trace_enabled,
                ),
                to_pulumi_object_field(
                    "loggingLevel",
                    &self.r#logging_level,
                ),
                to_pulumi_object_field(
                    "metricsEnabled",
                    &self.r#metrics_enabled,
                ),
                to_pulumi_object_field(
                    "requireAuthorizationForCacheControl",
                    &self.r#require_authorization_for_cache_control,
                ),
                to_pulumi_object_field(
                    "throttlingBurstLimit",
                    &self.r#throttling_burst_limit,
                ),
                to_pulumi_object_field(
                    "throttlingRateLimit",
                    &self.r#throttling_rate_limit,
                ),
                to_pulumi_object_field(
                    "unauthorizedCacheControlHeaderStrategy",
                    &self.r#unauthorized_cache_control_header_strategy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MethodSettingsSettings {
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
                    r#cache_data_encrypted: {
                        let field_value = match fields_map.get("cacheDataEncrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheDataEncrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_ttl_in_seconds: {
                        let field_value = match fields_map.get("cacheTtlInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheTtlInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caching_enabled: {
                        let field_value = match fields_map.get("cachingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cachingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_trace_enabled: {
                        let field_value = match fields_map.get("dataTraceEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataTraceEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_level: {
                        let field_value = match fields_map.get("loggingLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'loggingLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metrics_enabled: {
                        let field_value = match fields_map.get("metricsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_authorization_for_cache_control: {
                        let field_value = match fields_map.get("requireAuthorizationForCacheControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireAuthorizationForCacheControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throttling_burst_limit: {
                        let field_value = match fields_map.get("throttlingBurstLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttlingBurstLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throttling_rate_limit: {
                        let field_value = match fields_map.get("throttlingRateLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttlingRateLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unauthorized_cache_control_header_strategy: {
                        let field_value = match fields_map.get("unauthorizedCacheControlHeaderStrategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'unauthorizedCacheControlHeaderStrategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
