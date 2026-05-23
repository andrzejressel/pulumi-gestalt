#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyRuleRateLimitOption {
    /// Can only be specified if the action for the rule is "rate_based_ban". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold.
    #[builder(into)]
    pub r#ban_duration_sec: i32,
    /// Can only be specified if the action for the rule is "rate_based_ban". If specified, the key will be banned for the configured 'banDurationSec' when the number of requests that exceed the 'rateLimitThreshold' also exceed this 'banThreshold'.
    #[builder(into)]
    pub r#ban_thresholds: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionBanThreshold>,
    /// Action to take for requests that are under the configured rate limit threshold. Valid option is "allow" only.
    #[builder(into)]
    pub r#conform_action: String,
    /// Determines the key to enforce the rateLimitThreshold on
    #[builder(into)]
    pub r#enforce_on_key: String,
    /// Enforce On Key Config of this security policy
    #[builder(into)]
    pub r#enforce_on_key_configs: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionEnforceOnKeyConfig>,
    /// Rate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value.
    #[builder(into)]
    pub r#enforce_on_key_name: String,
    /// Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are "deny()" where valid values for status are 403, 404, 429, and 502, and "redirect" where the redirect parameters come from exceedRedirectOptions below.
    #[builder(into)]
    pub r#exceed_action: String,
    /// Parameters defining the redirect action that is used as the exceed action. Cannot be specified if the exceed action is not redirect.
    #[builder(into)]
    pub r#exceed_redirect_options: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionExceedRedirectOption>,
    /// Threshold at which to begin ratelimiting.
    #[builder(into)]
    pub r#rate_limit_thresholds: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOptionRateLimitThreshold>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyRuleRateLimitOption {
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
                    "banThresholds",
                    &self.r#ban_thresholds,
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
                    "exceedRedirectOptions",
                    &self.r#exceed_redirect_options,
                ),
                to_pulumi_object_field(
                    "rateLimitThresholds",
                    &self.r#rate_limit_thresholds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("banDurationSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'banDurationSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ban_thresholds: {
                        let field_value = match fields_map.get("banThresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'banThresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#exceed_redirect_options: {
                        let field_value = match fields_map.get("exceedRedirectOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exceedRedirectOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_limit_thresholds: {
                        let field_value = match fields_map.get("rateLimitThresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'rateLimitThresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
