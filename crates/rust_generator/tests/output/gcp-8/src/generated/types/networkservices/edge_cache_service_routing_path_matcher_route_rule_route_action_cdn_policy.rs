#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicy {
    /// Enable signature generation or propagation on this route.
    /// This field may only be specified when signedRequestMode is set to REQUIRE_TOKENS.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "addSignatures")]
    pub r#add_signatures: Option<Box<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyAddSignatures>>,
    /// Defines the request parameters that contribute to the cache key.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cacheKeyPolicy")]
    pub r#cache_key_policy: Option<Box<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyCacheKeyPolicy>>,
    /// Cache modes allow users to control the behaviour of the cache, what content it should cache automatically, whether to respect origin headers, or whether to unconditionally cache all responses.
    /// For all cache modes, Cache-Control headers will be passed to the client. Use clientTtl to override what is sent to the client.
    /// Possible values are: `CACHE_ALL_STATIC`, `USE_ORIGIN_HEADERS`, `FORCE_CACHE_ALL`, `BYPASS_CACHE`.
    #[builder(into)]
    #[serde(rename = "cacheMode")]
    pub r#cache_mode: Option<String>,
    /// Specifies a separate client (e.g. browser client) TTL, separate from the TTL used by the edge caches. Leaving this empty will use the same cache TTL for both the CDN and the client-facing response.
    /// - The TTL must be > 0 and <= 86400s (1 day)
    /// - The clientTtl cannot be larger than the defaultTtl (if set)
    /// - Fractions of a second are not allowed.
    /// Omit this field to use the defaultTtl, or the max-age set by the origin, as the client-facing TTL.
    /// When the cache mode is set to "USE_ORIGIN_HEADERS" or "BYPASS_CACHE", you must omit this field.
    /// A duration in seconds terminated by 's'. Example: "3s".
    #[builder(into)]
    #[serde(rename = "clientTtl")]
    pub r#client_ttl: Option<String>,
    /// Specifies the default TTL for cached content served by this origin for responses that do not have an existing valid TTL (max-age or s-max-age).
    /// Defaults to 3600s (1 hour).
    /// - The TTL must be >= 0 and <= 31,536,000 seconds (1 year)
    /// - Setting a TTL of "0" means "always revalidate" (equivalent to must-revalidate)
    /// - The value of defaultTTL cannot be set to a value greater than that of maxTTL.
    /// - Fractions of a second are not allowed.
    /// - When the cacheMode is set to FORCE_CACHE_ALL, the defaultTTL will overwrite the TTL set in all responses.
    /// Note that infrequently accessed objects may be evicted from the cache before the defined TTL. Objects that expire will be revalidated with the origin.
    /// When the cache mode is set to "USE_ORIGIN_HEADERS" or "BYPASS_CACHE", you must omit this field.
    /// A duration in seconds terminated by 's'. Example: "3s".
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Option<String>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    /// Defaults to 86400s (1 day).
    /// Cache directives that attempt to set a max-age or s-maxage higher than this, or an Expires header more than maxTtl seconds in the future will be capped at the value of maxTTL, as if it were the value of an s-maxage Cache-Control directive.
    /// - The TTL must be >= 0 and <= 31,536,000 seconds (1 year)
    /// - Setting a TTL of "0" means "always revalidate"
    /// - The value of maxTtl must be equal to or greater than defaultTtl.
    /// - Fractions of a second are not allowed.
    /// When the cache mode is set to "USE_ORIGIN_HEADERS", "FORCE_CACHE_ALL", or "BYPASS_CACHE", you must omit this field.
    /// A duration in seconds terminated by 's'. Example: "3s".
    #[builder(into)]
    #[serde(rename = "maxTtl")]
    pub r#max_ttl: Option<String>,
    /// Negative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects. This can reduce the load on your origin and improve end-user experience by reducing response latency.
    /// By default, the CDNPolicy will apply the following default TTLs to these status codes:
    /// - HTTP 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m
    /// - HTTP 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s
    /// - HTTP 405 (Method Not Found), 414 (URI Too Long), 501 (Not Implemented): 60s
    /// These defaults can be overridden in negativeCachingPolicy
    #[builder(into)]
    #[serde(rename = "negativeCaching")]
    pub r#negative_caching: Option<bool>,
    /// Sets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.
    /// - Omitting the policy and leaving negativeCaching enabled will use the default TTLs for each status code, defined in negativeCaching.
    /// - TTLs must be >= 0 (where 0 is "always revalidate") and <= 86400s (1 day)
    /// Note that when specifying an explicit negativeCachingPolicy, you should take care to specify a cache TTL for all response codes that you wish to cache. The CDNPolicy will not apply any default negative caching when a policy exists.
    #[builder(into)]
    #[serde(rename = "negativeCachingPolicy")]
    pub r#negative_caching_policy: Option<std::collections::HashMap<String, String>>,
    /// The EdgeCacheKeyset containing the set of public keys used to validate signed requests at the edge.
    #[builder(into)]
    #[serde(rename = "signedRequestKeyset")]
    pub r#signed_request_keyset: Option<String>,
    /// Limit how far into the future the expiration time of a signed request may be.
    /// When set, a signed request is rejected if its expiration time is later than now + signedRequestMaximumExpirationTtl, where now is the time at which the signed request is first handled by the CDN.
    /// - The TTL must be > 0.
    /// - Fractions of a second are not allowed.
    /// By default, signedRequestMaximumExpirationTtl is not set and the expiration time of a signed request may be arbitrarily far into future.
    #[builder(into)]
    #[serde(rename = "signedRequestMaximumExpirationTtl")]
    pub r#signed_request_maximum_expiration_ttl: Option<String>,
    /// Whether to enforce signed requests. The default value is DISABLED, which means all content is public, and does not authorize access.
    /// You must also set a signedRequestKeyset to enable signed requests.
    /// When set to REQUIRE_SIGNATURES, all matching requests will have their signature validated. Requests that were not signed with the corresponding private key, or that are otherwise invalid (expired, do not match the signature, IP address, or header) will be rejected with a HTTP 403 and (if enabled) logged.
    /// Possible values are: `DISABLED`, `REQUIRE_SIGNATURES`, `REQUIRE_TOKENS`.
    #[builder(into)]
    #[serde(rename = "signedRequestMode")]
    pub r#signed_request_mode: Option<String>,
    /// Additional options for signed tokens.
    /// signedTokenOptions may only be specified when signedRequestMode is REQUIRE_TOKENS.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "signedTokenOptions")]
    pub r#signed_token_options: Option<Box<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicySignedTokenOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicy {
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
                    "add_signatures",
                    &self.r#add_signatures,
                ),
                to_pulumi_object_field(
                    "cache_key_policy",
                    &self.r#cache_key_policy,
                ),
                to_pulumi_object_field(
                    "cache_mode",
                    &self.r#cache_mode,
                ),
                to_pulumi_object_field(
                    "client_ttl",
                    &self.r#client_ttl,
                ),
                to_pulumi_object_field(
                    "default_ttl",
                    &self.r#default_ttl,
                ),
                to_pulumi_object_field(
                    "max_ttl",
                    &self.r#max_ttl,
                ),
                to_pulumi_object_field(
                    "negative_caching",
                    &self.r#negative_caching,
                ),
                to_pulumi_object_field(
                    "negative_caching_policy",
                    &self.r#negative_caching_policy,
                ),
                to_pulumi_object_field(
                    "signed_request_keyset",
                    &self.r#signed_request_keyset,
                ),
                to_pulumi_object_field(
                    "signed_request_maximum_expiration_ttl",
                    &self.r#signed_request_maximum_expiration_ttl,
                ),
                to_pulumi_object_field(
                    "signed_request_mode",
                    &self.r#signed_request_mode,
                ),
                to_pulumi_object_field(
                    "signed_token_options",
                    &self.r#signed_token_options,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicy {
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
                    r#add_signatures: {
                        let field_value = match fields_map.get("add_signatures") {
                            Some(value) => value,
                            None => bail!("Missing field 'add_signatures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#negative_caching_policy: {
                        let field_value = match fields_map.get("negative_caching_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'negative_caching_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed_request_keyset: {
                        let field_value = match fields_map.get("signed_request_keyset") {
                            Some(value) => value,
                            None => bail!("Missing field 'signed_request_keyset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed_request_maximum_expiration_ttl: {
                        let field_value = match fields_map.get("signed_request_maximum_expiration_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'signed_request_maximum_expiration_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed_request_mode: {
                        let field_value = match fields_map.get("signed_request_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'signed_request_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed_token_options: {
                        let field_value = match fields_map.get("signed_token_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'signed_token_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
