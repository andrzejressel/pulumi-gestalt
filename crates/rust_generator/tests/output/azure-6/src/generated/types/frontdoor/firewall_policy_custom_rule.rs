#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyCustomRule {
    /// The action to perform when the rule is matched. Possible values are `Allow`, `Block`, `Log`, or `Redirect`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Is the rule is enabled or disabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// One or more `match_condition` block defined below. Can support up to `10` `match_condition` blocks.
    #[builder(into)]
    #[serde(rename = "matchConditions")]
    pub r#match_conditions: Option<Vec<super::super::types::frontdoor::FirewallPolicyCustomRuleMatchCondition>>,
    /// Gets name of the resource that is unique within a policy. This name can be used to access the resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The priority of the rule. Rules with a lower value will be evaluated before rules with a higher value. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// The rate limit duration in minutes. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "rateLimitDurationInMinutes")]
    pub r#rate_limit_duration_in_minutes: Option<i32>,
    /// The rate limit threshold. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "rateLimitThreshold")]
    pub r#rate_limit_threshold: Option<i32>,
    /// The type of rule. Possible values are `MatchRule` or `RateLimitRule`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
