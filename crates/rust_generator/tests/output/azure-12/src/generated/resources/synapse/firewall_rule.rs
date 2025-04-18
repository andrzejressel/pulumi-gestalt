/// Allows you to Manages a Synapse Firewall Rule.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       identity:
///         type: SystemAssigned
///   exampleFirewallRule:
///     type: azure:synapse:FirewallRule
///     name: example
///     properties:
///       name: AllowAll
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 255.255.255.255
/// ```
///
/// ## Import
///
/// Synapse Firewall Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/firewallRule:FirewallRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.Synapse/workspaces/workspace1/firewallRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// The ending IP address to allow through the firewall for this rule.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0`.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` requires the `name` to be `AllowAllWindowsAzureIps`.
        #[builder(into)]
        pub end_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the firewall rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The starting IP address to allow through the firewall for this rule.
        #[builder(into)]
        pub start_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Synapse Workspace on which to create the Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// The ending IP address to allow through the firewall for this rule.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0`.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` requires the `name` to be `AllowAllWindowsAzureIps`.
        pub end_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The Name of the firewall rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The starting IP address to allow through the firewall for this rule.
        pub start_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Synapse Workspace on which to create the Firewall Rule. Changing this forces a new resource to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallRuleArgs,
    ) -> FirewallRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let end_ip_address_binding = args.end_ip_address.get_output(context);
        let name_binding = args.name.get_output(context);
        let start_ip_address_binding = args.start_ip_address.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/firewallRule:FirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endIpAddress".into(),
                    value: &end_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startIpAddress".into(),
                    value: &start_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallRuleResult {
            end_ip_address: o.get_field("endIpAddress"),
            name: o.get_field("name"),
            start_ip_address: o.get_field("startIpAddress"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
        }
    }
}
