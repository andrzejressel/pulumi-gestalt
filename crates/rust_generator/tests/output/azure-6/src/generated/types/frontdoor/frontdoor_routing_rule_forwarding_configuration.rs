#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRoutingRuleForwardingConfiguration {
    /// Specifies the name of the Backend Pool to forward the incoming traffic to.
    #[builder(into)]
    #[serde(rename = "backendPoolName")]
    pub r#backend_pool_name: String,
    /// Specify the minimum caching duration (in ISO8601 notation e.g. `P1DT2H` for 1 day and 2 hours). Needs to be greater than 0 and smaller than 365 days. `cache_duration` works only in combination with `cache_enabled` set to `true`.
    #[builder(into)]
    #[serde(rename = "cacheDuration")]
    pub r#cache_duration: Option<String>,
    /// Specifies whether to Enable caching or not. Valid options are `true` or `false`. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "cacheEnabled")]
    pub r#cache_enabled: Option<bool>,
    /// Defines cache behaviour in relation to query string parameters. Valid options are `StripAll`, `StripAllExcept`, `StripOnly` or `StripNone`. Defaults to `StripAll`.
    #[builder(into)]
    #[serde(rename = "cacheQueryParameterStripDirective")]
    pub r#cache_query_parameter_strip_directive: Option<String>,
    /// Specify query parameters (array). Works only in combination with `cache_query_parameter_strip_directive` set to `StripAllExcept` or `StripOnly`.
    #[builder(into)]
    #[serde(rename = "cacheQueryParameters")]
    pub r#cache_query_parameters: Option<Vec<String>>,
    /// Whether to use dynamic compression when caching. Valid options are `true` or `false`. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "cacheUseDynamicCompression")]
    pub r#cache_use_dynamic_compression: Option<bool>,
    /// Path to use when constructing the request to forward to the backend. This functions as a URL Rewrite. Default behaviour preserves the URL path.
    #[builder(into)]
    #[serde(rename = "customForwardingPath")]
    pub r#custom_forwarding_path: Option<String>,
    /// Protocol to use when redirecting. Valid options are `HttpOnly`, `HttpsOnly`, or `MatchRequest`. Defaults to `HttpsOnly`.
    #[builder(into)]
    #[serde(rename = "forwardingProtocol")]
    pub r#forwarding_protocol: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRoutingRuleForwardingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backend_pool_name",
                    &self.r#backend_pool_name,
                ),
                to_pulumi_object_field(
                    "cache_duration",
                    &self.r#cache_duration,
                ),
                to_pulumi_object_field(
                    "cache_enabled",
                    &self.r#cache_enabled,
                ),
                to_pulumi_object_field(
                    "cache_query_parameter_strip_directive",
                    &self.r#cache_query_parameter_strip_directive,
                ),
                to_pulumi_object_field(
                    "cache_query_parameters",
                    &self.r#cache_query_parameters,
                ),
                to_pulumi_object_field(
                    "cache_use_dynamic_compression",
                    &self.r#cache_use_dynamic_compression,
                ),
                to_pulumi_object_field(
                    "custom_forwarding_path",
                    &self.r#custom_forwarding_path,
                ),
                to_pulumi_object_field(
                    "forwarding_protocol",
                    &self.r#forwarding_protocol,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRoutingRuleForwardingConfiguration {
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
                    r#backend_pool_name: {
                        let field_value = match fields_map.get("backend_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_duration: {
                        let field_value = match fields_map.get("cache_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_enabled: {
                        let field_value = match fields_map.get("cache_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_query_parameter_strip_directive: {
                        let field_value = match fields_map.get("cache_query_parameter_strip_directive") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_query_parameter_strip_directive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_query_parameters: {
                        let field_value = match fields_map.get("cache_query_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_query_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_use_dynamic_compression: {
                        let field_value = match fields_map.get("cache_use_dynamic_compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_use_dynamic_compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_forwarding_path: {
                        let field_value = match fields_map.get("custom_forwarding_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_forwarding_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarding_protocol: {
                        let field_value = match fields_map.get("forwarding_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarding_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
