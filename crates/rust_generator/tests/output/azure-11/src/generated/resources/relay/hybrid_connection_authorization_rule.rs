/// Manages an Azure Relay Hybrid Connection Authorization Rule.
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
///   exampleNamespace:
///     type: azure:relay:Namespace
///     name: example
///     properties:
///       name: example-relay
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Standard
///       tags:
///         source: terraform
///   exampleHybridConnection:
///     type: azure:relay:HybridConnection
///     name: example
///     properties:
///       name: acctestrnhc-%d
///       resourceGroupName: ${example.name}
///       relayNamespaceName: ${exampleNamespace.name}
///       requiresClientAuthorization: false
///       userMetadata: testmetadata
///   exampleHybridConnectionAuthorizationRule:
///     type: azure:relay:HybridConnectionAuthorizationRule
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       hybridConnectionName: ${exampleHybridConnection.name}
///       namespaceName: ${exampleNamespace.name}
///       listen: true
///       send: true
///       manage: false
/// ```
///
/// ## Import
///
/// Azure Relay Hybrid Connection Authorization Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:relay/hybridConnectionAuthorizationRule:HybridConnectionAuthorizationRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Relay/namespaces/namespace1/hybridConnections/connection1/authorizationRules/rule1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod hybrid_connection_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HybridConnectionAuthorizationRuleArgs {
        /// Name of the Azure Relay Hybrid Connection for which this Azure Relay Hybrid Connection Authorization Rule will be created. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        #[builder(into)]
        pub hybrid_connection_name: pulumi_gestalt_rust::Input<String>,
        /// Grants listen access to this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Grants manage access to this Authorization Rule. When this property is `true` - both `listen` and `send` must be set to `true` too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The name which should be used for this Azure Relay Hybrid Connection Authorization Rule. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Name of the Azure Relay Namespace for which this Azure Relay Hybrid Connection Authorization Rule will be created. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the Resource Group where the Azure Relay Hybrid Connection Authorization Rule should exist. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Grants send access to this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_gestalt_rust::Input<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct HybridConnectionAuthorizationRuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Name of the Azure Relay Hybrid Connection for which this Azure Relay Hybrid Connection Authorization Rule will be created. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        pub hybrid_connection_name: pulumi_gestalt_rust::Output<String>,
        /// Grants listen access to this Authorization Rule. Defaults to `false`.
        pub listen: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Grants manage access to this Authorization Rule. When this property is `true` - both `listen` and `send` must be set to `true` too. Defaults to `false`.
        pub manage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Relay Hybrid Connection Authorization Rule. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of the Azure Relay Namespace for which this Azure Relay Hybrid Connection Authorization Rule will be created. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Connection String for the Azure Relay Hybrid Connection Authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the Azure Relay Hybrid Connection Authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Relay Hybrid Connection Authorization Rule should exist. Changing this forces a new Azure Relay Hybrid Connection Authorization Rule to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Connection String for the Azure Relay Hybrid Connection Authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the Azure Relay Hybrid Connection Authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Grants send access to this Authorization Rule. Defaults to `false`.
        pub send: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridConnectionAuthorizationRuleArgs,
    ) -> HybridConnectionAuthorizationRuleResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridConnectionAuthorizationRuleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> HybridConnectionAuthorizationRuleResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridConnectionAuthorizationRuleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> HybridConnectionAuthorizationRuleResult {
        let hybrid_connection_name_binding = args.hybrid_connection_name.get_output(ctx);
        let listen_binding = args.listen.get_output(ctx);
        let manage_binding = args.manage.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let namespace_name_binding = args.namespace_name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let send_binding = args.send.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:relay/hybridConnectionAuthorizationRule:HybridConnectionAuthorizationRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hybridConnectionName".into(),
                    value: &hybrid_connection_name_binding.drop_type(),
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
        HybridConnectionAuthorizationRuleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            hybrid_connection_name: o.get_field("hybridConnectionName"),
            listen: o.get_field("listen"),
            manage: o.get_field("manage"),
            name: o.get_field("name"),
            namespace_name: o.get_field("namespaceName"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            primary_key: o.get_field("primaryKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            secondary_key: o.get_field("secondaryKey"),
            send: o.get_field("send"),
        }
    }
}
