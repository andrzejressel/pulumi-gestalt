#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WaitingRoomRulesRule {
    /// Action to perform in the ruleset rule. Available values: `bypass_waiting_room`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Brief summary of the waiting room rule and its intended use.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Criteria for an HTTP request to trigger the waiting room rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Waiting Room Rules Docs](https://developers.cloudflare.com/waiting-room/additional-options/waiting-room-rules/bypass-rules/).
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: String,
    /// Unique rule identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Whether the rule is enabled or disabled. Available values: `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Version of the waiting room rule.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
