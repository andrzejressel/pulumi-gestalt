#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyManagedRulesManagedRuleSetRuleGroupOverrideRule {
    /// Describes the override action to be applied when rule matches. Possible values are `Allow`, `AnomalyScoring`, `Block`, `JSChallenge` and `Log`. `JSChallenge` is only valid for rulesets of type `Microsoft_BotManagerRuleSet`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// Describes if the managed rule is in enabled state or disabled state. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Identifier for the managed rule.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
}
