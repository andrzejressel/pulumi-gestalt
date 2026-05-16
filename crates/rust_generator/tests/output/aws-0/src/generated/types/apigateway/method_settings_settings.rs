#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MethodSettingsSettings {
    /// Whether the cached responses are encrypted.
    #[builder(into)]
    #[serde(rename = "cacheDataEncrypted")]
    pub r#cache_data_encrypted: Option<bool>,
    /// Time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached.
    #[builder(into)]
    #[serde(rename = "cacheTtlInSeconds")]
    pub r#cache_ttl_in_seconds: Option<i32>,
    /// Whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached.
    #[builder(into)]
    #[serde(rename = "cachingEnabled")]
    pub r#caching_enabled: Option<bool>,
    /// Whether data trace logging is enabled for this method, which effects the log entries pushed to Amazon CloudWatch Logs.
    #[builder(into)]
    #[serde(rename = "dataTraceEnabled")]
    pub r#data_trace_enabled: Option<bool>,
    /// Logging level for this method, which effects the log entries pushed to Amazon CloudWatch Logs. The available levels are `OFF`, `ERROR`, and `INFO`.
    #[builder(into)]
    #[serde(rename = "loggingLevel")]
    pub r#logging_level: Option<String>,
    /// Whether Amazon CloudWatch metrics are enabled for this method.
    #[builder(into)]
    #[serde(rename = "metricsEnabled")]
    pub r#metrics_enabled: Option<bool>,
    /// Whether authorization is required for a cache invalidation request.
    #[builder(into)]
    #[serde(rename = "requireAuthorizationForCacheControl")]
    pub r#require_authorization_for_cache_control: Option<bool>,
    /// Throttling burst limit. Default: `-1` (throttling disabled).
    #[builder(into)]
    #[serde(rename = "throttlingBurstLimit")]
    pub r#throttling_burst_limit: Option<i32>,
    /// Throttling rate limit. Default: `-1` (throttling disabled).
    #[builder(into)]
    #[serde(rename = "throttlingRateLimit")]
    pub r#throttling_rate_limit: Option<f64>,
    /// How to handle unauthorized requests for cache invalidation. The available values are `FAIL_WITH_403`, `SUCCEED_WITH_RESPONSE_HEADER`, `SUCCEED_WITHOUT_RESPONSE_HEADER`.
    #[builder(into)]
    #[serde(rename = "unauthorizedCacheControlHeaderStrategy")]
    pub r#unauthorized_cache_control_header_strategy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MethodSettingsSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cache_data_encrypted".to_string(), self.r#cache_data_encrypted.to_pulumi_value().await);
            map.insert("cache_ttl_in_seconds".to_string(), self.r#cache_ttl_in_seconds.to_pulumi_value().await);
            map.insert("caching_enabled".to_string(), self.r#caching_enabled.to_pulumi_value().await);
            map.insert("data_trace_enabled".to_string(), self.r#data_trace_enabled.to_pulumi_value().await);
            map.insert("logging_level".to_string(), self.r#logging_level.to_pulumi_value().await);
            map.insert("metrics_enabled".to_string(), self.r#metrics_enabled.to_pulumi_value().await);
            map.insert("require_authorization_for_cache_control".to_string(), self.r#require_authorization_for_cache_control.to_pulumi_value().await);
            map.insert("throttling_burst_limit".to_string(), self.r#throttling_burst_limit.to_pulumi_value().await);
            map.insert("throttling_rate_limit".to_string(), self.r#throttling_rate_limit.to_pulumi_value().await);
            map.insert("unauthorized_cache_control_header_strategy".to_string(), self.r#unauthorized_cache_control_header_strategy.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MethodSettingsSettings {
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
                    r#cache_data_encrypted: {
                        let field_value = match fields_map.get("cache_data_encrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_data_encrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cache_ttl_in_seconds: {
                        let field_value = match fields_map.get("cache_ttl_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_ttl_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#caching_enabled: {
                        let field_value = match fields_map.get("caching_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'caching_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#data_trace_enabled: {
                        let field_value = match fields_map.get("data_trace_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_trace_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#metrics_enabled: {
                        let field_value = match fields_map.get("metrics_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'metrics_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_authorization_for_cache_control: {
                        let field_value = match fields_map.get("require_authorization_for_cache_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_authorization_for_cache_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
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
                    r#unauthorized_cache_control_header_strategy: {
                        let field_value = match fields_map.get("unauthorized_cache_control_header_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'unauthorized_cache_control_header_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
