#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupNatRuleCollection {
    /// The action to take for the NAT rules in this collection. Currently, the only possible value is `Dnat`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The name which should be used for this NAT rule collection.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The priority of the NAT rule collection. The range is `100` - `65000`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// A `nat_rule` block as defined below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Vec<super::super::types::network::FirewallPolicyRuleCollectionGroupNatRuleCollectionRule>,
}
