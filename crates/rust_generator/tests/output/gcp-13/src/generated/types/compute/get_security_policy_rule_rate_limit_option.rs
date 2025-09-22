#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
