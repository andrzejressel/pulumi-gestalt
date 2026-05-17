#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyRuleRateLimitOptions {
    /// Can only be specified if the action for the rule is "rate_based_ban".
    /// If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold.
    #[builder(into)]
    #[serde(rename = "banDurationSec")]
    pub r#ban_duration_sec: Option<i32>,
    /// Can only be specified if the action for the rule is "rate_based_ban".
    /// If specified, the key will be banned for the configured 'banDurationSec' when the number of requests that exceed the 'rateLimitThreshold' also exceed this 'banThreshold'.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "banThreshold")]
    pub r#ban_threshold: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptionsBanThreshold>>,
    /// Action to take for requests that are under the configured rate limit threshold.
    /// Valid option is "allow" only.
    #[builder(into)]
    #[serde(rename = "conformAction")]
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
    #[serde(rename = "enforceOnKey")]
    pub r#enforce_on_key: Option<String>,
    /// If specified, any combination of values of enforceOnKeyType/enforceOnKeyName is treated as the key on which ratelimit threshold/action is enforced.
    /// You can specify up to 3 enforceOnKeyConfigs.
    /// If enforceOnKeyConfigs is specified, enforceOnKey must not be specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "enforceOnKeyConfigs")]
    pub r#enforce_on_key_configs: Option<Vec<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptionsEnforceOnKeyConfig>>,
    /// Rate limit key name applicable only for the following key types:
    /// HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.
    /// HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value.
    #[builder(into)]
    #[serde(rename = "enforceOnKeyName")]
    pub r#enforce_on_key_name: Option<String>,
    /// Action to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.
    /// Valid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502.
    #[builder(into)]
    #[serde(rename = "exceedAction")]
    pub r#exceed_action: Option<String>,
    /// Threshold at which to begin ratelimiting.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rateLimitThreshold")]
    pub r#rate_limit_threshold: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptionsRateLimitThreshold>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionSecurityPolicyRuleRateLimitOptions {
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
                    "ban_duration_sec",
                    &self.r#ban_duration_sec,
                ),
                to_pulumi_object_field(
                    "ban_threshold",
                    &self.r#ban_threshold,
                ),
                to_pulumi_object_field(
                    "conform_action",
                    &self.r#conform_action,
                ),
                to_pulumi_object_field(
                    "enforce_on_key",
                    &self.r#enforce_on_key,
                ),
                to_pulumi_object_field(
                    "enforce_on_key_configs",
                    &self.r#enforce_on_key_configs,
                ),
                to_pulumi_object_field(
                    "enforce_on_key_name",
                    &self.r#enforce_on_key_name,
                ),
                to_pulumi_object_field(
                    "exceed_action",
                    &self.r#exceed_action,
                ),
                to_pulumi_object_field(
                    "rate_limit_threshold",
                    &self.r#rate_limit_threshold,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("ban_duration_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'ban_duration_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ban_threshold: {
                        let field_value = match fields_map.get("ban_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'ban_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conform_action: {
                        let field_value = match fields_map.get("conform_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'conform_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_on_key: {
                        let field_value = match fields_map.get("enforce_on_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_on_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_on_key_configs: {
                        let field_value = match fields_map.get("enforce_on_key_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_on_key_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_on_key_name: {
                        let field_value = match fields_map.get("enforce_on_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_on_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exceed_action: {
                        let field_value = match fields_map.get("exceed_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'exceed_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_limit_threshold: {
                        let field_value = match fields_map.get("rate_limit_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_limit_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
