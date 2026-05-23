#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyRuleRateLimitOptions {
    /// Can only be specified if the action for the rule is "rate_based_ban".
    /// If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold.
    #[builder(into)]
    pub r#ban_duration_sec: Option<i32>,
    /// Can only be specified if the action for the rule is "rate_based_ban".
    /// If specified, the key will be banned for the configured 'banDurationSec' when the number of requests that exceed the 'rateLimitThreshold' also exceed this 'banThreshold'.
    /// Structure is documented below.
    #[builder(into)]
    pub r#ban_threshold: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptionsBanThreshold>>,
    /// Action to take for requests that are under the configured rate limit threshold.
    /// Valid option is "allow" only.
    #[builder(into)]
    pub r#conform_action: Option<String>,
    /// Determines the key to enforce the rateLimitThreshold on. Possible values are:
    /// * ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if "enforceOnKey" is not configured.
    /// * IP: The source IP address of the request is the key. Each IP has this limit enforced separately.
    /// * HTTP_HEADER: The value of the HTTP header whose name is configured under "enforceOnKeyName". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.
    /// * XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.
    /// * HTTP_COOKIE: The value of the HTTP cookie whose name is configured under "enforceOnKeyName". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.
    /// * HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.
    /// * SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.
    /// * REGION_CODE: The country/region from which the request originates.
    /// * TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.
    /// * USER_IP: The IP address of the originating client, which is resolved based on "userIpRequestHeaders" configured with the security policy. If there is no "userIpRequestHeaders" configuration or an IP address cannot be resolved from it, the key type defaults to IP.
    /// Possible values are: `ALL`, `IP`, `HTTP_HEADER`, `XFF_IP`, `HTTP_COOKIE`, `HTTP_PATH`, `SNI`, `REGION_CODE`, `TLS_JA3_FINGERPRINT`, `USER_IP`.
    #[builder(into)]
    pub r#enforce_on_key: Option<String>,
    /// If specified, any combination of values of enforceOnKeyType/enforceOnKeyName is treated as the key on which ratelimit threshold/action is enforced.
    /// You can specify up to 3 enforceOnKeyConfigs.
    /// If enforceOnKeyConfigs is specified, enforceOnKey must not be specified.
    /// Structure is documented below.
    #[builder(into)]
    pub r#enforce_on_key_configs: Option<Vec<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptionsEnforceOnKeyConfig>>,
    /// Rate limit key name applicable only for the following key types:
    /// HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.
    /// HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value.
    #[builder(into)]
    pub r#enforce_on_key_name: Option<String>,
    /// Action to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.
    /// Valid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502.
    #[builder(into)]
    pub r#exceed_action: Option<String>,
    /// Threshold at which to begin ratelimiting.
    /// Structure is documented below.
    #[builder(into)]
    pub r#rate_limit_threshold: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptionsRateLimitThreshold>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionSecurityPolicyRuleRateLimitOptions {
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
                    "banDurationSec",
                    &self.r#ban_duration_sec,
                ),
                to_pulumi_object_field(
                    "banThreshold",
                    &self.r#ban_threshold,
                ),
                to_pulumi_object_field(
                    "conformAction",
                    &self.r#conform_action,
                ),
                to_pulumi_object_field(
                    "enforceOnKey",
                    &self.r#enforce_on_key,
                ),
                to_pulumi_object_field(
                    "enforceOnKeyConfigs",
                    &self.r#enforce_on_key_configs,
                ),
                to_pulumi_object_field(
                    "enforceOnKeyName",
                    &self.r#enforce_on_key_name,
                ),
                to_pulumi_object_field(
                    "exceedAction",
                    &self.r#exceed_action,
                ),
                to_pulumi_object_field(
                    "rateLimitThreshold",
                    &self.r#rate_limit_threshold,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionSecurityPolicyRuleRateLimitOptions {
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
                    r#ban_duration_sec: {
                        let field_value = match fields_map.get("banDurationSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'banDurationSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ban_threshold: {
                        let field_value = match fields_map.get("banThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'banThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conform_action: {
                        let field_value = match fields_map.get("conformAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'conformAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_on_key: {
                        let field_value = match fields_map.get("enforceOnKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforceOnKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_on_key_configs: {
                        let field_value = match fields_map.get("enforceOnKeyConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforceOnKeyConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_on_key_name: {
                        let field_value = match fields_map.get("enforceOnKeyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforceOnKeyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exceed_action: {
                        let field_value = match fields_map.get("exceedAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'exceedAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_limit_threshold: {
                        let field_value = match fields_map.get("rateLimitThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'rateLimitThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
