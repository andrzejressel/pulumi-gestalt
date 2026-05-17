#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyCacheKeyPolicy {
    /// If true, requests to different hosts will be cached separately.
    /// Note: this should only be enabled if hosts share the same origin and content. Removing the host from the cache key may inadvertently result in different objects being cached than intended, depending on which route the first user matched.
    #[builder(into)]
    #[serde(rename = "excludeHost")]
    pub r#exclude_host: Option<bool>,
    /// If true, exclude query string parameters from the cache key
    /// If false (the default), include the query string parameters in
    /// the cache key according to includeQueryParameters and
    /// excludeQueryParameters. If neither includeQueryParameters nor
    /// excludeQueryParameters is set, the entire query string will be
    /// included.
    #[builder(into)]
    #[serde(rename = "excludeQueryString")]
    pub r#exclude_query_string: Option<bool>,
    /// Names of query string parameters to exclude from cache keys. All other parameters will be included.
    /// Either specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters.
    #[builder(into)]
    #[serde(rename = "excludedQueryParameters")]
    pub r#excluded_query_parameters: Option<Vec<String>>,
    /// If true, http and https requests will be cached separately.
    #[builder(into)]
    #[serde(rename = "includeProtocol")]
    pub r#include_protocol: Option<bool>,
    /// Names of Cookies to include in cache keys.  The cookie name and cookie value of each cookie named will be used as part of the cache key.
    /// Cookie names:
    /// - must be valid RFC 6265 "cookie-name" tokens
    /// - are case sensitive
    /// - cannot start with "Edge-Cache-" (case insensitive)
    /// Note that specifying several cookies, and/or cookies that have a large range of values (e.g., per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.
    /// You may specify up to three cookie names.
    #[builder(into)]
    #[serde(rename = "includedCookieNames")]
    pub r#included_cookie_names: Option<Vec<String>>,
    /// Names of HTTP request headers to include in cache keys. The value of the header field will be used as part of the cache key.
    /// - Header names must be valid HTTP RFC 7230 header field values.
    /// - Header field names are case insensitive
    /// - To include the HTTP method, use ":method"
    /// Note that specifying several headers, and/or headers that have a large range of values (e.g. per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.
    #[builder(into)]
    #[serde(rename = "includedHeaderNames")]
    pub r#included_header_names: Option<Vec<String>>,
    /// Names of query string parameters to include in cache keys. All other parameters will be excluded.
    /// Either specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters.
    #[builder(into)]
    #[serde(rename = "includedQueryParameters")]
    pub r#included_query_parameters: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyCacheKeyPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "exclude_host".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_host,
                )
                .await,
            );
            map.insert(
                "exclude_query_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_query_string,
                )
                .await,
            );
            map.insert(
                "excluded_query_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_query_parameters,
                )
                .await,
            );
            map.insert(
                "include_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_protocol,
                )
                .await,
            );
            map.insert(
                "included_cookie_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_cookie_names,
                )
                .await,
            );
            map.insert(
                "included_header_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_header_names,
                )
                .await,
            );
            map.insert(
                "included_query_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_query_parameters,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyCacheKeyPolicy {
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
                    r#exclude_host: {
                        let field_value = match fields_map.get("exclude_host") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_query_string: {
                        let field_value = match fields_map.get("exclude_query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_query_parameters: {
                        let field_value = match fields_map.get("excluded_query_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_query_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_protocol: {
                        let field_value = match fields_map.get("include_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_cookie_names: {
                        let field_value = match fields_map.get("included_cookie_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_cookie_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_header_names: {
                        let field_value = match fields_map.get("included_header_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_header_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_query_parameters: {
                        let field_value = match fields_map.get("included_query_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_query_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
