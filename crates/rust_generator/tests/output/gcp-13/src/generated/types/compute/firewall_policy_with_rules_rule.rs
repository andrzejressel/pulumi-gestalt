#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyWithRulesRule {
    /// The Action to perform when the client connection triggers the rule. Can currently be either
    /// "allow", "deny", "apply_security_profile_group" or "goto_next".
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// A description of the rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The direction in which this rule applies. If unspecified an INGRESS rule is created.
    /// Possible values are: `INGRESS`, `EGRESS`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    /// Denotes whether the firewall policy rule is disabled. When set to true,
    /// the firewall policy rule is not enforced and traffic behaves as if it did
    /// not exist. If this is unspecified, the firewall policy rule will be
    /// enabled.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Denotes whether to enable logging for a particular rule.
    /// If logging is enabled, logs will be exported to the
    /// configured export destination in Stackdriver.
    #[builder(into)]
    #[serde(rename = "enableLogging")]
    pub r#enable_logging: Option<bool>,
    /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::compute::FirewallPolicyWithRulesRuleMatch>,
    /// An integer indicating the priority of a rule in the list. The priority must be a value
    /// between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the
    /// highest priority and 2147483647 is the lowest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// An optional name for the rule. This field is not a unique identifier
    /// and can be updated.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: Option<String>,
    /// A fully-qualified URL of a SecurityProfile resource instance.
    /// Example:
    /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
    /// Must be specified if action is 'apply_security_profile_group'.
    #[builder(into)]
    #[serde(rename = "securityProfileGroup")]
    pub r#security_profile_group: Option<String>,
    /// A list of network resource URLs to which this rule applies.
    /// This field allows you to control which network's VMs get
    /// this rule. If this field is left blank, all VMs
    /// within the organization will receive the rule.
    #[builder(into)]
    #[serde(rename = "targetResources")]
    pub r#target_resources: Option<Vec<String>>,
    /// A list of service accounts indicating the sets of
    /// instances that are applied with this rule.
    #[builder(into)]
    #[serde(rename = "targetServiceAccounts")]
    pub r#target_service_accounts: Option<Vec<String>>,
    /// Boolean flag indicating if the traffic should be TLS decrypted.
    /// It can be set only if action = 'apply_security_profile_group' and cannot be set for other actions.
    #[builder(into)]
    #[serde(rename = "tlsInspect")]
    pub r#tls_inspect: Option<bool>,
}
