/// Manages a Palo Alto Local Rulestack Prefix List.
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
///             .name("rg-example")
///             .build_struct(),
///     );
///     let exampleLocalRulestack = local_rulestack::create(
///         "exampleLocalRulestack",
///         LocalRulestackArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLocalRulestackPrefixList = local_rulestack_prefix_list::create(
///         "exampleLocalRulestackPrefixList",
///         LocalRulestackPrefixListArgs::builder()
///             .name("example")
///             .prefix_lists(vec!["10.0.1.0/24",])
///             .rulestack_id("${exampleLocalRulestack.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Palo Alto Local Rulestack Prefix Lists can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/localRulestackPrefixList:LocalRulestackPrefixList example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/localRulestacks/myLocalRulestack/prefixLists/myFQDNList1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_rulestack_prefix_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackPrefixListArgs {
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the Prefix List.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack Prefix List.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Prefixes.
        #[builder(into)]
        pub prefix_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the Local Rulestack on which to create this Prefix List. Changing this forces a new Palo Alto Local Rulestack Prefix List to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackPrefixListResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for the Prefix List.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack Prefix List.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of Prefixes.
        pub prefix_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Local Rulestack on which to create this Prefix List. Changing this forces a new Palo Alto Local Rulestack Prefix List to be created.
        pub rulestack_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalRulestackPrefixListArgs,
    ) -> LocalRulestackPrefixListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_comment_binding = args.audit_comment.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let prefix_lists_binding = args.prefix_lists.get_output(context);
        let rulestack_id_binding = args.rulestack_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackPrefixList:LocalRulestackPrefixList"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditComment".into(),
                    value: &audit_comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefixLists".into(),
                    value: &prefix_lists_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalRulestackPrefixListResult {
            id: o.get_field("id"),
            audit_comment: o.get_field("auditComment"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            prefix_lists: o.get_field("prefixLists"),
            rulestack_id: o.get_field("rulestackId"),
        }
    }
}
