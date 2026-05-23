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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bypassCacheOnRequestHeaders",
                    &self.r#bypass_cache_on_request_headers,
                ),
                to_pulumi_object_field(
                    "cacheKeyPolicy",
                    &self.r#cache_key_policy,
                ),
                to_pulumi_object_field(
                    "cacheMode",
                    &self.r#cache_mode,
                ),
                to_pulumi_object_field(
                    "clientTtl",
                    &self.r#client_ttl,
                ),
                to_pulumi_object_field(
                    "defaultTtl",
                    &self.r#default_ttl,
                ),
                to_pulumi_object_field(
                    "maxTtl",
                    &self.r#max_ttl,
                ),
                to_pulumi_object_field(
                    "negativeCaching",
                    &self.r#negative_caching,
                ),
                to_pulumi_object_field(
                    "negativeCachingPolicies",
                    &self.r#negative_caching_policies,
                ),
                to_pulumi_object_field(
                    "requestCoalescing",
                    &self.r#request_coalescing,
                ),
                to_pulumi_object_field(
                    "serveWhileStale",
                    &self.r#serve_while_stale,
                ),
                to_pulumi_object_field(
                    "signedUrlCacheMaxAgeSec",
                    &self.r#signed_url_cache_max_age_sec,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("bypassCacheOnRequestHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypassCacheOnRequestHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_key_policy: {
                        let field_value = match fields_map.get("cacheKeyPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheKeyPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_mode: {
                        let field_value = match fields_map.get("cacheMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_ttl: {
                        let field_value = match fields_map.get("clientTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_ttl: {
                        let field_value = match fields_map.get("defaultTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ttl: {
                        let field_value = match fields_map.get("maxTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negative_caching: {
                        let field_value = match fields_map.get("negativeCaching") {
                            Some(value) => value,
                            None => bail!("Missing field 'negativeCaching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negative_caching_policies: {
                        let field_value = match fields_map.get("negativeCachingPolicies") {
                            Some(value) => value,
                            None => bail!("Missing field 'negativeCachingPolicies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_coalescing: {
                        let field_value = match fields_map.get("requestCoalescing") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestCoalescing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serve_while_stale: {
                        let field_value = match fields_map.get("serveWhileStale") {
                            Some(value) => value,
                            None => bail!("Missing field 'serveWhileStale' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed_url_cache_max_age_sec: {
                        let field_value = match fields_map.get("signedUrlCacheMaxAgeSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'signedUrlCacheMaxAgeSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
