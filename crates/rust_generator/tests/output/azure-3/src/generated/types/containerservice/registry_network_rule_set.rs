#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryNetworkRuleSet {
    /// The behaviour for requests matching no rules. Either `Allow` or `Deny`. Defaults to `Allow`
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Option<String>,
    /// One or more `ip_rule` blocks as defined below.
    /// 
    /// > **NOTE:** `network_rule_set` is only supported with the `Premium` SKU at this time.
    /// 
    /// > **NOTE:** Azure automatically configures Network Rules - to remove these you'll need to specify an `network_rule_set` block with `default_action` set to `Deny`.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<super::super::types::containerservice::RegistryNetworkRuleSetIpRule>>,
}
