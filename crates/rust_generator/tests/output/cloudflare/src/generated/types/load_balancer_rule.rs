#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerRule {
    /// The statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules).
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<String>,
    /// A disabled rule will not be executed.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Settings for a HTTP response to return directly to the eyeball if the condition is true. Note: `overrides` or `fixed_response` must be set.
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Option<Box<super::types::LoadBalancerRuleFixedResponse>>,
    /// Human readable name for this rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The load balancer settings to alter if this rule's `condition` is true. Note: `overrides` or `fixed_response` must be set.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Option<Vec<super::types::LoadBalancerRuleOverride>>,
    /// Priority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// Terminates indicates that if this rule is true no further rules should be executed. Note: setting a `fixed_response` forces this field to `true`.
    #[builder(into)]
    #[serde(rename = "terminates")]
    pub r#terminates: Option<bool>,
}
