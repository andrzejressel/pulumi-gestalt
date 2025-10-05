/// A Response Policy Rule is a selector that applies its behavior to queries that match the selector.
/// Selectors are DNS names, which may be wildcards or exact matches.
/// Each DNS query subject to a Response Policy matches at most one ResponsePolicyRule,
/// as identified by the dns_name field with the longest matching suffix.
///
///
///
/// ## Example Usage
///
/// ### Dns Response Policy Rule Basic
///
///
/// ```yaml
/// resources:
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   network-2:
///     type: gcp:compute:Network
///     properties:
///       name: network-2
///       autoCreateSubnetworks: false
///   response-policy:
///     type: gcp:dns:ResponsePolicy
///     properties:
///       responsePolicyName: example-response-policy
///       networks:
///         - networkUrl: ${["network-1"].id}
///         - networkUrl: ${["network-2"].id}
///   example-response-policy-rule:
///     type: gcp:dns:ResponsePolicyRule
///     properties:
///       responsePolicy: ${["response-policy"].responsePolicyName}
///       ruleName: example-rule
///       dnsName: dns.example.com.
///       localData:
///         localDatas:
///           - name: dns.example.com.
///             type: A
///             ttl: 300
///             rrdatas:
///               - 192.0.2.91
/// ```
///
/// ## Import
///
/// ResponsePolicyRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/responsePolicies/{{response_policy}}/rules/{{rule_name}}`
///
/// * `{{project}}/{{response_policy}}/{{rule_name}}`
///
/// * `{{response_policy}}/{{rule_name}}`
///
/// When using the `pulumi import` command, ResponsePolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicyRule:ResponsePolicyRule default projects/{{project}}/responsePolicies/{{response_policy}}/rules/{{rule_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicyRule:ResponsePolicyRule default {{project}}/{{response_policy}}/{{rule_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicyRule:ResponsePolicyRule default {{response_policy}}/{{rule_name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod response_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponsePolicyRuleArgs {
        /// Answer this query with a behavior rather than DNS data. Acceptable values are 'behaviorUnspecified', and 'bypassResponsePolicy'
        #[builder(into, default)]
        pub behavior: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule.
        #[builder(into)]
        pub dns_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name;
        /// in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed.
        /// Structure is documented below.
        #[builder(into, default)]
        pub local_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dns::ResponsePolicyRuleLocalData>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifies the response policy addressed by this request.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub response_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An identifier for this rule. Must be unique with the ResponsePolicy.
        #[builder(into)]
        pub rule_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResponsePolicyRuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Answer this query with a behavior rather than DNS data. Acceptable values are 'behaviorUnspecified', and 'bypassResponsePolicy'
        pub behavior: pulumi_gestalt_rust::Output<Option<String>>,
        /// The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name;
        /// in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed.
        /// Structure is documented below.
        pub local_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::dns::ResponsePolicyRuleLocalData>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Identifies the response policy addressed by this request.
        ///
        ///
        /// - - -
        pub response_policy: pulumi_gestalt_rust::Output<String>,
        /// An identifier for this rule. Must be unique with the ResponsePolicy.
        pub rule_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResponsePolicyRuleArgs,
    ) -> ResponsePolicyRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let behavior_binding = args.behavior.get_output(context);
        let dns_name_binding = args.dns_name.get_output(context);
        let local_data_binding = args.local_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let response_policy_binding = args.response_policy.get_output(context);
        let rule_name_binding = args.rule_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dns/responsePolicyRule:ResponsePolicyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "behavior".into(),
                    value: &behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsName".into(),
                    value: &dns_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localData".into(),
                    value: &local_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responsePolicy".into(),
                    value: &response_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResponsePolicyRuleResult {
            id: o.get_field("id"),
            behavior: o.get_field("behavior"),
            dns_name: o.get_field("dnsName"),
            local_data: o.get_field("localData"),
            project: o.get_field("project"),
            response_policy: o.get_field("responsePolicy"),
            rule_name: o.get_field("ruleName"),
        }
    }
}
