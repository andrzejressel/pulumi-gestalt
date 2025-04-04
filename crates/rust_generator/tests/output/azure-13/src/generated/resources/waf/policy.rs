/// Manages a Azure Web Application Firewall Policy instance.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let examplePolicy = policy::create(
///         "examplePolicy",
///         PolicyArgs::builder()
///             .custom_rules(
///                 vec![
///                     PolicyCustomRule::builder().action("Block")
///                     .matchConditions(vec![PolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24", "10.0.0.0/24",])
///                     .matchVariables(vec![PolicyCustomRuleMatchConditionMatchVariable::builder()
///                     .variableName("RemoteAddr").build_struct(),])
///                     .negationCondition(false).operator("IPMatch").build_struct(),])
///                     .name("Rule1").priority(1).ruleType("MatchRule").build_struct(),
///                     PolicyCustomRule::builder().action("Block")
///                     .matchConditions(vec![PolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24",])
///                     .matchVariables(vec![PolicyCustomRuleMatchConditionMatchVariable::builder()
///                     .variableName("RemoteAddr").build_struct(),])
///                     .negationCondition(false).operator("IPMatch").build_struct(),
///                     PolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["Windows",])
///                     .matchVariables(vec![PolicyCustomRuleMatchConditionMatchVariable::builder()
///                     .selector("UserAgent").variableName("RequestHeaders")
///                     .build_struct(),]).negationCondition(false).operator("Contains")
///                     .build_struct(),]).name("Rule2").priority(2).ruleType("MatchRule")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .managed_rules(
///                 PolicyManagedRules::builder()
///                     .exclusions(
///                         vec![
///                             PolicyManagedRulesExclusion::builder()
///                             .matchVariable("RequestHeaderNames")
///                             .selector("x-company-secret-header")
///                             .selectorMatchOperator("Equals").build_struct(),
///                             PolicyManagedRulesExclusion::builder()
///                             .matchVariable("RequestCookieNames").selector("too-tasty")
///                             .selectorMatchOperator("EndsWith").build_struct(),
///                         ],
///                     )
///                     .managedRuleSets(
///                         vec![
///                             PolicyManagedRulesManagedRuleSet::builder()
///                             .ruleGroupOverrides(vec![PolicyManagedRulesManagedRuleSetRuleGroupOverride::builder()
///                             .ruleGroupName("REQUEST-920-PROTOCOL-ENFORCEMENT")
///                             .rules(vec![PolicyManagedRulesManagedRuleSetRuleGroupOverrideRule::builder()
///                             .action("Log").enabled(true).id("920300").build_struct(),
///                             PolicyManagedRulesManagedRuleSetRuleGroupOverrideRule::builder()
///                             .action("Block").enabled(true).id("920440").build_struct(),])
///                             .build_struct(),]). type ("OWASP").version("3.2")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example-wafpolicy")
///             .policy_settings(
///                 PolicyPolicySettings::builder()
///                     .enabled(true)
///                     .fileUploadLimitInMb(100)
///                     .maxRequestBodySizeInKb(128)
///                     .mode("Prevention")
///                     .requestBodyCheck(true)
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Web Application Firewall Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:waf/policy:Policy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Network/applicationGatewayWebApplicationFirewallPolicies/example-wafpolicy
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// One or more `custom_rules` blocks as defined below.
        #[builder(into, default)]
        pub custom_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::waf::PolicyCustomRule>>,
        >,
        /// Resource location. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `managed_rules` blocks as defined below.
        #[builder(into)]
        pub managed_rules: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::waf::PolicyManagedRules,
        >,
        /// The name of the policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `policy_settings` block as defined below.
        #[builder(into, default)]
        pub policy_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::waf::PolicyPolicySettings>,
        >,
        /// The name of the resource group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Web Application Firewall Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// One or more `custom_rules` blocks as defined below.
        pub custom_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::waf::PolicyCustomRule>>,
        >,
        /// A list of HTTP Listener IDs from an `azure.network.ApplicationGateway`.
        pub http_listener_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Resource location. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `managed_rules` blocks as defined below.
        pub managed_rules: pulumi_gestalt_rust::Output<
            super::super::types::waf::PolicyManagedRules,
        >,
        /// The name of the policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of URL Path Map Path Rule IDs from an `azure.network.ApplicationGateway`.
        pub path_based_rule_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `policy_settings` block as defined below.
        pub policy_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::waf::PolicyPolicySettings>,
        >,
        /// The name of the resource group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Web Application Firewall Policy.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_rules_binding = args.custom_rules.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_rules_binding = args.managed_rules.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_settings_binding = args.policy_settings.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:waf/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customRules".into(),
                    value: &custom_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedRules".into(),
                    value: &managed_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policySettings".into(),
                    value: &policy_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            custom_rules: o.get_field("customRules"),
            http_listener_ids: o.get_field("httpListenerIds"),
            location: o.get_field("location"),
            managed_rules: o.get_field("managedRules"),
            name: o.get_field("name"),
            path_based_rule_ids: o.get_field("pathBasedRuleIds"),
            policy_settings: o.get_field("policySettings"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
