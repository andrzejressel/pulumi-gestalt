/// ## Example Usage
///
/// ### Compute Firewall Policy With Rules Full
///
///
/// ```yaml
/// resources:
///   firewall-policy-with-rules:
///     type: gcp:compute:FirewallPolicyWithRules
///     properties:
///       shortName: tf-fw-org-policy-with-rules
///       description: Terraform test
///       parent: organizations/123456789
///       rules:
///         - description: tcp rule
///           priority: 1000
///           enableLogging: true
///           action: allow
///           direction: EGRESS
///           match:
///             layer4Configs:
///               - ipProtocol: tcp
///                 ports:
///                   - 8080
///                   - 7070
///             destIpRanges:
///               - 11.100.0.1/32
///             destFqdns:
///               - www.yyy.com
///               - www.zzz.com
///             destRegionCodes:
///               - HK
///               - IN
///             destThreatIntelligences:
///               - iplist-search-engines-crawlers
///               - iplist-tor-exit-nodes
///             destAddressGroups:
///               - ${addressGroup1.id}
///           targetResources:
///             - https://www.googleapis.com/compute/beta/projects/${project.name}/global/networks/default
///         - description: udp rule
///           priority: 2000
///           enableLogging: false
///           action: deny
///           direction: INGRESS
///           match:
///             layer4Configs:
///               - ipProtocol: udp
///             srcIpRanges:
///               - 0.0.0.0/0
///             srcFqdns:
///               - www.abc.com
///               - www.def.com
///             srcRegionCodes:
///               - US
///               - CA
///             srcThreatIntelligences:
///               - iplist-known-malicious-ips
///               - iplist-public-clouds
///             srcAddressGroups:
///               - ${addressGroup1.id}
///           disabled: true
///         - description: security profile group rule
///           ruleName: tcp rule
///           priority: 3000
///           enableLogging: false
///           action: apply_security_profile_group
///           direction: INGRESS
///           match:
///             layer4Configs:
///               - ipProtocol: tcp
///             srcIpRanges:
///               - 0.0.0.0/0
///           targetServiceAccounts:
///             - test@google.com
///           securityProfileGroup: //networksecurity.googleapis.com/${securityProfileGroup1.id}
///           tlsInspect: true
///   addressGroup1:
///     type: gcp:networksecurity:AddressGroup
///     name: address_group_1
///     properties:
///       name: tf-address-group
///       parent: organizations/123456789
///       description: Global address group
///       location: global
///       items:
///         - 208.80.154.224/32
///       type: IPV4
///       capacity: 100
///   securityProfileGroup1:
///     type: gcp:networksecurity:SecurityProfileGroup
///     name: security_profile_group_1
///     properties:
///       name: tf-security-profile-group
///       parent: organizations/123456789
///       description: my description
///       threatPreventionProfile: ${securityProfile1.id}
///   securityProfile1:
///     type: gcp:networksecurity:SecurityProfile
///     name: security_profile_1
///     properties:
///       name: tf-security-profile
///       type: THREAT_PREVENTION
///       parent: organizations/123456789
///       location: global
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// FirewallPolicyWithRules can be imported using any of these accepted formats:
///
/// * `locations/global/firewallPolicies/{{policy_id}}`
///
/// * `{{policy_id}}`
///
/// When using the `pulumi import` command, FirewallPolicyWithRules can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicyWithRules:FirewallPolicyWithRules default locations/global/firewallPolicies/{{policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicyWithRules:FirewallPolicyWithRules default {{policy_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy_with_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyWithRulesArgs {
        /// (Output)
        /// A description of the rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of this FirewallPolicy in the Cloud Resource Hierarchy.
        /// Format: organizations/{organization_id} or folders/{folder_id}
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of firewall policy rules.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::FirewallPolicyWithRulesRule>,
        >,
        /// A textual name of the security policy.
        #[builder(into)]
        pub short_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyWithRulesResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// A description of the rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fingerprint of the resource. This field is used internally during updates of this resource.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The parent of this FirewallPolicy in the Cloud Resource Hierarchy.
        /// Format: organizations/{organization_id} or folders/{folder_id}
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// A list of pre-define firewall policy rules.
        /// Structure is documented below.
        pub predefined_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::FirewallPolicyWithRulesPredefinedRule>,
        >,
        /// Total count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples.
        pub rule_tuple_count: pulumi_gestalt_rust::Output<i32>,
        /// A list of firewall policy rules.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::FirewallPolicyWithRulesRule>,
        >,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL for this resource with the resource id.
        pub self_link_with_id: pulumi_gestalt_rust::Output<String>,
        /// A textual name of the security policy.
        pub short_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallPolicyWithRulesArgs,
    ) -> FirewallPolicyWithRulesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let short_name_binding = args.short_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/firewallPolicyWithRules:FirewallPolicyWithRules".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallPolicyWithRulesResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            parent: o.get_field("parent"),
            policy_id: o.get_field("policyId"),
            predefined_rules: o.get_field("predefinedRules"),
            rule_tuple_count: o.get_field("ruleTupleCount"),
            rules: o.get_field("rules"),
            self_link: o.get_field("selfLink"),
            self_link_with_id: o.get_field("selfLinkWithId"),
            short_name: o.get_field("shortName"),
        }
    }
}
