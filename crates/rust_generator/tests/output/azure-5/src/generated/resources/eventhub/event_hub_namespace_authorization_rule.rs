/// Manages an Authorization Rule for an Event Hub Namespace.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: resourcegroup
///       location: West Europe
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: acceptanceTestEventHubNamespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Basic
///       capacity: 2
///       tags:
///         environment: Production
///   exampleEventHubNamespaceAuthorizationRule:
///     type: azure:eventhub:EventHubNamespaceAuthorizationRule
///     name: example
///     properties:
///       name: navi
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       listen: true
///       send: false
///       manage: false
/// ```
///
/// ## Import
///
/// EventHub Namespace Authorization Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/eventHubNamespaceAuthorizationRule:EventHubNamespaceAuthorizationRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/authorizationRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_hub_namespace_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventHubNamespaceAuthorizationRuleArgs {
        /// Grants listen access to this this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Grants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Authorization Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Grants send access to this this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EventHubNamespaceAuthorizationRuleResult {
        /// Grants listen access to this this Authorization Rule. Defaults to `false`.
        pub listen: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Grants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        pub manage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Authorization Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Connection String for the Authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the Primary Connection String for the Authorization Rule, which is generated when disaster recovery is enabled.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the Authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Connection String for the Authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the Secondary Connection String for the Authorization Rule, which is generated when disaster recovery is enabled.
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the Authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Grants send access to this this Authorization Rule. Defaults to `false`.
        pub send: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventHubNamespaceAuthorizationRuleArgs,
    ) -> EventHubNamespaceAuthorizationRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let listen_binding = args.listen.get_output(context);
        let manage_binding = args.manage.get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let send_binding = args.send.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/eventHubNamespaceAuthorizationRule:EventHubNamespaceAuthorizationRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listen".into(),
                    value: &listen_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manage".into(),
                    value: &manage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "send".into(),
                    value: &send_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventHubNamespaceAuthorizationRuleResult {
            listen: o.get_field("listen"),
            manage: o.get_field("manage"),
            name: o.get_field("name"),
            namespace_name: o.get_field("namespaceName"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            primary_connection_string_alias: o.get_field("primaryConnectionStringAlias"),
            primary_key: o.get_field("primaryKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            secondary_connection_string_alias: o
                .get_field("secondaryConnectionStringAlias"),
            secondary_key: o.get_field("secondaryKey"),
            send: o.get_field("send"),
        }
    }
}
