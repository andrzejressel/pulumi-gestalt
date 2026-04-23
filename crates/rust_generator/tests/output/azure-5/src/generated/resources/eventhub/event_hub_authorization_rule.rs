/// Manages a Event Hubs authorization Rule within an Event Hub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
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
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: acceptanceTestEventHub
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 2
///       messageRetention: 2
///   exampleAuthorizationRule:
///     type: azure:eventhub:AuthorizationRule
///     name: example
///     properties:
///       name: navi
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///       listen: true
///       send: false
///       manage: false
/// ```
///
/// ## Import
///
/// EventHub Authorization Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/eventHubAuthorizationRule:EventHubAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/authorizationRules/rule1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod event_hub_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventHubAuthorizationRuleArgs {
        /// Specifies the name of the EventHub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::Input<String>,
        /// Does this Authorization Rule have permissions to Listen to the Event Hub? Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Does this Authorization Rule have permissions to Manage to the Event Hub? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Specifies the name of the EventHub Authorization Rule resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Does this Authorization Rule have permissions to Send to the Event Hub? Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_gestalt_rust::Input<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EventHubAuthorizationRuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the EventHub. Changing this forces a new resource to be created.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// Does this Authorization Rule have permissions to Listen to the Event Hub? Defaults to `false`.
        pub listen: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Does this Authorization Rule have permissions to Manage to the Event Hub? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        pub manage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the EventHub Authorization Rule resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Connection String for the Event Hubs authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the Primary Connection String for the Event Hubs authorization Rule, which is generated when disaster recovery is enabled.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the Event Hubs authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Connection String for the Event Hubs Authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the Secondary Connection String for the Event Hubs Authorization Rule, which is generated when disaster recovery is enabled.
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the Event Hubs Authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Does this Authorization Rule have permissions to Send to the Event Hub? Defaults to `false`.
        pub send: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventHubAuthorizationRuleArgs,
    ) -> EventHubAuthorizationRuleResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventHubAuthorizationRuleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EventHubAuthorizationRuleResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventHubAuthorizationRuleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EventHubAuthorizationRuleResult {
        let eventhub_name_binding = args.eventhub_name.get_output(ctx);
        let listen_binding = args.listen.get_output(ctx);
        let manage_binding = args.manage.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let namespace_name_binding = args.namespace_name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let send_binding = args.send.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/eventHubAuthorizationRule:EventHubAuthorizationRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding.drop_type(),
                },
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
            options,
        };
        let o = ctx.register_resource(request);
        EventHubAuthorizationRuleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            eventhub_name: o.get_field("eventhubName"),
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
