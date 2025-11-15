#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyRulePreconfiguredWafConfigExclusion {
    /// Request cookie whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestCookies")]
    pub r#request_cookies: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestCooky>>,
    /// Request header whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestHeader>>,
    /// Request query parameter whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Note that the parameter can be in the query string or in the POST body.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestQueryParams")]
    pub r#request_query_params: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam>>,
    /// Request URI from the request line to be excluded from inspection during preconfigured WAF evaluation.
    /// When specifying this field, the query or fragment part should be excluded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestUris")]
    pub r#request_uris: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestUri>>,
    /// A list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.
    /// If omitted, it refers to all the rule IDs under the WAF rule set.
    #[builder(into)]
    #[serde(rename = "targetRuleIds")]
    pub r#target_rule_ids: Option<Vec<String>>,
    /// Target WAF rule set to apply the preconfigured WAF exclusion.
    #[builder(into)]
    #[serde(rename = "targetRuleSet")]
    pub r#target_rule_set: String,
}
