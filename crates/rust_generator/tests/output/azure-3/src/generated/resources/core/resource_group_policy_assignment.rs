/// Manages a Resource Group Policy Assignment.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleDefinition = definition::create(
///         "exampleDefinition",
///         DefinitionArgs::builder()
///             .display_name("my-policy-definition")
///             .mode("All")
///             .name("only-deploy-in-westeurope")
///             .policy_rule(
///                 " {\n    \"if\": {\n      \"not\": {\n        \"field\": \"location\",\n        \"equals\": \"westeurope\"\n      }\n    },\n    \"then\": {\n      \"effect\": \"Deny\"\n    }\n  }\n",
///             )
///             .policy_type("Custom")
///             .build_struct(),
///     );
///     let exampleResourceGroupPolicyAssignment = resource_group_policy_assignment::create(
///         "exampleResourceGroupPolicyAssignment",
///         ResourceGroupPolicyAssignmentArgs::builder()
///             .name("example")
///             .parameters(
///                 "    {\n      \"tagName\": {\n        \"value\": \"Business Unit\"\n      },\n      \"tagValue\": {\n        \"value\": \"BU\"\n      }\n    }",
///             )
///             .policy_definition_id("${exampleDefinition.id}")
///             .resource_group_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Resource Group Policy Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourceGroupPolicyAssignment:ResourceGroupPolicyAssignment example /subscriptions/00000000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Authorization/policyAssignments/assignment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_group_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupPolicyAssignmentArgs {
        /// A description which should be used for this Policy Assignment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Display Name for this Policy Assignment.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        #[builder(into, default)]
        pub enforce: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::core::ResourceGroupPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON mapping of any Metadata for this Policy.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Policy Assignment. Changing this forces a new Policy Assignment to be created. Cannot exceed 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `non_compliance_message` blocks as defined below.
        #[builder(into, default)]
        pub non_compliance_messages: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::ResourceGroupPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        #[builder(into, default)]
        pub not_scopes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        #[builder(into, default)]
        pub overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::ResourceGroupPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub policy_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Resource Group where this Policy Assignment should be created. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        #[builder(into, default)]
        pub resource_selectors: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::ResourceGroupPolicyAssignmentResourceSelector,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupPolicyAssignmentResult {
        /// A description which should be used for this Policy Assignment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Display Name for this Policy Assignment.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        pub enforce: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::core::ResourceGroupPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Policy Assignment. Changing this forces a new Policy Assignment to be created. Cannot exceed 64 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `non_compliance_message` blocks as defined below.
        pub non_compliance_messages: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourceGroupPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        pub not_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        pub overrides: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        pub policy_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource Group where this Policy Assignment should be created. Changing this forces a new Policy Assignment to be created.
        pub resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        pub resource_selectors: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourceGroupPolicyAssignmentResourceSelector,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceGroupPolicyAssignmentArgs,
    ) -> ResourceGroupPolicyAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enforce_binding = args.enforce.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let non_compliance_messages_binding = args
            .non_compliance_messages
            .get_output(context);
        let not_scopes_binding = args.not_scopes.get_output(context);
        let overrides_binding = args.overrides.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let policy_definition_id_binding = args.policy_definition_id.get_output(context);
        let resource_group_id_binding = args.resource_group_id.get_output(context);
        let resource_selectors_binding = args.resource_selectors.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/resourceGroupPolicyAssignment:ResourceGroupPolicyAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enforce".into(),
                    value: &enforce_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nonComplianceMessages".into(),
                    value: &non_compliance_messages_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notScopes".into(),
                    value: &not_scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrides".into(),
                    value: &overrides_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDefinitionId".into(),
                    value: &policy_definition_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupId".into(),
                    value: &resource_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSelectors".into(),
                    value: &resource_selectors_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceGroupPolicyAssignmentResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enforce: o.get_field("enforce"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            non_compliance_messages: o.get_field("nonComplianceMessages"),
            not_scopes: o.get_field("notScopes"),
            overrides: o.get_field("overrides"),
            parameters: o.get_field("parameters"),
            policy_definition_id: o.get_field("policyDefinitionId"),
            resource_group_id: o.get_field("resourceGroupId"),
            resource_selectors: o.get_field("resourceSelectors"),
        }
    }
}
