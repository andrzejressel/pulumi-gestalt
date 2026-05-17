#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRuleActionsRouteConfigurationOverrideAction {
    /// `HonorOrigin` the Front Door will always honor origin response header directive. If the origin directive is missing, Front Door will cache contents anywhere from `1` to `3` days. `OverrideAlways` the TTL value returned from your Front Door Origin is overwritten with the value specified in the action. This behavior will only be applied if the response is cacheable. `OverrideIfOriginMissing` if no TTL value gets returned from your Front Door Origin, the rule sets the TTL to the value specified in the action. This behavior will only be applied if the response is cacheable. `Disabled` the Front Door will not cache the response contents, irrespective of Front Door Origin response directives. Possible values include `HonorOrigin`, `OverrideAlways`, `OverrideIfOriginMissing` or `Disabled`.
    #[builder(into)]
    #[serde(rename = "cacheBehavior")]
    pub r#cache_behavior: Option<String>,
    /// When Cache behavior is set to `Override` or `SetIfMissing`, this field specifies the cache duration to use. The maximum duration is 366 days specified in the `d.HH:MM:SS` format(e.g. `365.23:59:59`). If the desired maximum cache duration is less than 1 day then the maximum cache duration should be specified in the `HH:MM:SS` format(e.g. `23:59:59`).
    #[builder(into)]
    #[serde(rename = "cacheDuration")]
    pub r#cache_duration: Option<String>,
    /// The Front Door Origin Group resource ID that the request should be routed to. This overrides the configuration specified in the Front Door Endpoint route.
    #[builder(into)]
    #[serde(rename = "cdnFrontdoorOriginGroupId")]
    pub r#cdn_frontdoor_origin_group_id: Option<String>,
    /// Should the Front Door dynamically compress the content? Possible values include `true` or `false`.
    /// 
    /// ->**NOTE:** Content won't be compressed on AzureFrontDoor when requested content is smaller than `1 byte` or larger than `1 MB`.
    #[builder(into)]
    #[serde(rename = "compressionEnabled")]
    pub r#compression_enabled: Option<bool>,
    /// The forwarding protocol the request will be redirected as. This overrides the configuration specified in the route to be associated with. Possible values include `MatchRequest`, `HttpOnly` or `HttpsOnly`.
    /// 
    /// ->**NOTE:** If the `cdn_frontdoor_origin_group_id` is not defined you cannot set the `forwarding_protocol`.
    #[builder(into)]
    #[serde(rename = "forwardingProtocol")]
    pub r#forwarding_protocol: Option<String>,
    /// `IncludeSpecifiedQueryStrings` query strings specified in the `query_string_parameters` field get included when the cache key gets generated. `UseQueryString` cache every unique URL, each unique URL will have its own cache key. `IgnoreSpecifiedQueryStrings` query strings specified in the `query_string_parameters` field get excluded when the cache key gets generated. `IgnoreQueryString` query strings aren't considered when the cache key gets generated. Possible values include `IgnoreQueryString`, `UseQueryString`, `IgnoreSpecifiedQueryStrings` or `IncludeSpecifiedQueryStrings`.
    #[builder(into)]
    #[serde(rename = "queryStringCachingBehavior")]
    pub r#query_string_caching_behavior: Option<String>,
    /// A list of query string parameter names.
    /// 
    /// ->**NOTE:** `query_string_parameters` is a required field when the `query_string_caching_behavior` is set to `IncludeSpecifiedQueryStrings` or `IgnoreSpecifiedQueryStrings`.
    #[builder(into)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRuleActionsRouteConfigurationOverrideAction {
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
                    "cache_behavior",
                    &self.r#cache_behavior,
                ),
                to_pulumi_object_field(
                    "cache_duration",
                    &self.r#cache_duration,
                ),
                to_pulumi_object_field(
                    "cdn_frontdoor_origin_group_id",
                    &self.r#cdn_frontdoor_origin_group_id,
                ),
                to_pulumi_object_field(
                    "compression_enabled",
                    &self.r#compression_enabled,
                ),
                to_pulumi_object_field(
                    "forwarding_protocol",
                    &self.r#forwarding_protocol,
                ),
                to_pulumi_object_field(
                    "query_string_caching_behavior",
                    &self.r#query_string_caching_behavior,
                ),
                to_pulumi_object_field(
                    "query_string_parameters",
                    &self.r#query_string_parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRuleActionsRouteConfigurationOverrideAction {
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
                    r#cache_behavior: {
                        let field_value = match fields_map.get("cache_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#cdn_frontdoor_origin_group_id: {
                        let field_value = match fields_map.get("cdn_frontdoor_origin_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdn_frontdoor_origin_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compression_enabled: {
                        let field_value = match fields_map.get("compression_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_string_caching_behavior: {
                        let field_value = match fields_map.get("query_string_caching_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_caching_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_parameters: {
                        let field_value = match fields_map.get("query_string_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
