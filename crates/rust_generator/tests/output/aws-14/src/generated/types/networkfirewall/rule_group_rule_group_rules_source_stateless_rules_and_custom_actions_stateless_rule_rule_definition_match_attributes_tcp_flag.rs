#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributesTcpFlag {
    /// Set of flags to look for in a packet. This setting can only specify values that are also specified in `masks`.
    /// Valid values: `FIN`, `SYN`, `RST`, `PSH`, `ACK`, `URG`, `ECE`, `CWR`.
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Vec<String>,
    /// Set of flags to consider in the inspection. To inspect all flags, leave this empty.
    /// Valid values: `FIN`, `SYN`, `RST`, `PSH`, `ACK`, `URG`, `ECE`, `CWR`.
    #[builder(into)]
    #[serde(rename = "masks")]
    pub r#masks: Option<Vec<String>>,
}
