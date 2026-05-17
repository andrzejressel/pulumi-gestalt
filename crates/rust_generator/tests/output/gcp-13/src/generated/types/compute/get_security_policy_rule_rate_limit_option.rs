#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyRuleRateLimitOption {
    /// Can only be specified if the action for the rule is "rate_based_ban". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold.
    #[builder(into)]
    #[serde(rename = "banDurationSec")]
    pub r#ban_duration_sec: i32,
    /// Can only be specified if the action for the rule is "rate_based_ban". If specified, the key will be banned for the configured 'banDurationSec' when the number of requests that exceed the 'rateLimitThreshold' also exceed this 'banThreshold'.
    #[builder(into)]
    #[serde(rename = "banThresholds")]
    pub r#ban_thresholds: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionBanThreshold>,
    /// Action to take for requests that are under the configured rate limit threshold. Valid option is "allow" only.
    #[builder(into)]
    #[serde(rename = "conformAction")]
    pub r#conform_action: String,
    /// Determines the key to enforce the rateLimitThreshold on
    #[builder(into)]
    #[serde(rename = "enforceOnKey")]
    pub r#enforce_on_key: String,
    /// Enforce On Key Config of this security policy
    #[builder(into)]
    #[serde(rename = "enforceOnKeyConfigs")]
    pub r#enforce_on_key_configs: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionEnforceOnKeyConfig>,
    /// Rate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value.
    #[builder(into)]
    #[serde(rename = "enforceOnKeyName")]
    pub r#enforce_on_key_name: String,
    /// Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are "deny()" where valid values for status are 403, 404, 429, and 502, and "redirect" where the redirect parameters come from exceedRedirectOptions below.
    #[builder(into)]
    #[serde(rename = "exceedAction")]
    pub r#exceed_action: String,
    /// Parameters defining the redirect action that is used as the exceed action. Cannot be specified if the exceed action is not redirect.
    #[builder(into)]
    #[serde(rename = "exceedRedirectOptions")]
    pub r#exceed_redirect_options: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionExceedRedirectOption>,
    /// Threshold at which to begin ratelimiting.
    #[builder(into)]
    #[serde(rename = "rateLimitThresholds")]
    pub r#rate_limit_thresholds: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionRateLimitThreshold>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyRuleRateLimitOption {
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
                "ban_duration_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ban_duration_sec,
                )
                .await,
            );
            map.insert(
                "ban_thresholds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ban_thresholds,
                )
                .await,
            );
            map.insert(
                "conform_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#conform_action,
                )
                .await,
            );
            map.insert(
                "enforce_on_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce_on_key,
                )
                .await,
            );
            map.insert(
                "enforce_on_key_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce_on_key_configs,
                )
                .await,
            );
            map.insert(
                "enforce_on_key_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce_on_key_name,
                )
                .await,
            );
            map.insert(
                "exceed_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exceed_action,
                )
                .await,
            );
            map.insert(
                "exceed_redirect_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exceed_redirect_options,
                )
                .await,
            );
            map.insert(
                "rate_limit_thresholds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_limit_thresholds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyRuleRateLimitOption {
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
                    r#ban_thresholds: {
                        let field_value = match fields_map.get("ban_thresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'ban_thresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#exceed_redirect_options: {
                        let field_value = match fields_map.get("exceed_redirect_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'exceed_redirect_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_limit_thresholds: {
                        let field_value = match fields_map.get("rate_limit_thresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_limit_thresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
