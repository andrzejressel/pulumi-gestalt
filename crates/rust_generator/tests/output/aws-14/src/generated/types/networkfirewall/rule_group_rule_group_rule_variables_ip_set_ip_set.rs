#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRuleVariablesIpSetIpSet {
    /// Set of IP addresses and address ranges, in CIDR notation.
    #[builder(into)]
    #[serde(rename = "definitions")]
    pub r#definitions: Vec<String>,
}
