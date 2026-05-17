#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendBucketCdnPolicy {
    /// Bypass the cache when the specified request headers are matched - e.g. Pragma or Authorization headers. Up to 5 headers can be specified. The cache is bypassed for all cdnPolicy.cacheMode settings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bypassCacheOnRequestHeaders")]
    pub r#bypass_cache_on_request_headers: Option<Vec<super::super::types::compute::BackendBucketCdnPolicyBypassCacheOnRequestHeader>>,
    /// The CacheKeyPolicy for this CdnPolicy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cacheKeyPolicy")]
    pub r#cache_key_policy: Option<Box<super::super::types::compute::BackendBucketCdnPolicyCacheKeyPolicy>>,
    /// Specifies the cache setting for all responses from this backend.
    /// The possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC
    /// Possible values are: `USE_ORIGIN_HEADERS`, `FORCE_CACHE_ALL`, `CACHE_ALL_STATIC`.
    #[builder(into)]
    #[serde(rename = "cacheMode")]
    pub r#cache_mode: Option<String>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    #[builder(into)]
    #[serde(rename = "clientTtl")]
    pub r#client_ttl: Option<i32>,
    /// Specifies the default TTL for cached content served by this origin for responses
    /// that do not have an existing valid TTL (max-age or s-max-age).
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Option<i32>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    #[builder(into)]
    #[serde(rename = "maxTtl")]
    pub r#max_ttl: Option<i32>,
    /// Negative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects.
    #[builder(into)]
    #[serde(rename = "negativeCaching")]
    pub r#negative_caching: Option<bool>,
    /// Sets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.
    /// Omitting the policy and leaving negativeCaching enabled will use Cloud CDN's default cache TTLs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "negativeCachingPolicies")]
    pub r#negative_caching_policies: Option<Vec<super::super::types::compute::BackendBucketCdnPolicyNegativeCachingPolicy>>,
    /// If true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests to the origin.
    #[builder(into)]
    #[serde(rename = "requestCoalescing")]
    pub r#request_coalescing: Option<bool>,
    /// Serve existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache.
    #[builder(into)]
    #[serde(rename = "serveWhileStale")]
    pub r#serve_while_stale: Option<i32>,
    /// Maximum number of seconds the response to a signed URL request will
    /// be considered fresh. After this time period,
    /// the response will be revalidated before being served.
    /// When serving responses to signed URL requests,
    /// Cloud CDN will internally behave as though
    /// all responses from this backend had a "Cache-Control: public,
    /// max-age=[TTL]" header, regardless of any existing Cache-Control
    /// header. The actual headers served in responses will not be altered.
    #[builder(into)]
    #[serde(rename = "signedUrlCacheMaxAgeSec")]
    pub r#signed_url_cache_max_age_sec: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendBucketCdnPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bypass_cache_on_request_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bypass_cache_on_request_headers,
                )
                .await,
            );
            map.insert(
                "cache_key_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_key_policy,
                )
                .await,
            );
            map.insert(
                "cache_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_mode,
                )
                .await,
            );
            map.insert(
                "client_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_ttl,
                )
                .await,
            );
            map.insert(
                "default_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_ttl,
                )
                .await,
            );
            map.insert(
                "max_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_ttl,
                )
                .await,
            );
            map.insert(
                "negative_caching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#negative_caching,
                )
                .await,
            );
            map.insert(
                "negative_caching_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#negative_caching_policies,
                )
                .await,
            );
            map.insert(
                "request_coalescing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_coalescing,
                )
                .await,
            );
            map.insert(
                "serve_while_stale".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serve_while_stale,
                )
                .await,
            );
            map.insert(
                "signed_url_cache_max_age_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signed_url_cache_max_age_sec,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendBucketCdnPolicy {
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
                    r#bypass_cache_on_request_headers: {
                        let field_value = match fields_map.get("bypass_cache_on_request_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypass_cache_on_request_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_key_policy: {
                        let field_value = match fields_map.get("cache_key_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_key_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_mode: {
                        let field_value = match fields_map.get("cache_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_ttl: {
                        let field_value = match fields_map.get("client_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_ttl: {
                        let field_value = match fields_map.get("default_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ttl: {
                        let field_value = match fields_map.get("max_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negative_caching: {
                        let field_value = match fields_map.get("negative_caching") {
                            Some(value) => value,
                            None => bail!("Missing field 'negative_caching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negative_caching_policies: {
                        let field_value = match fields_map.get("negative_caching_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'negative_caching_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_coalescing: {
                        let field_value = match fields_map.get("request_coalescing") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_coalescing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serve_while_stale: {
                        let field_value = match fields_map.get("serve_while_stale") {
                            Some(value) => value,
                            None => bail!("Missing field 'serve_while_stale' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed_url_cache_max_age_sec: {
                        let field_value = match fields_map.get("signed_url_cache_max_age_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'signed_url_cache_max_age_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
