/// Manages a Synapse Managed Private Endpoint.
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
///       managedVirtualNetworkEnabled: true
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
///   exampleConnect:
///     type: azure:storage:Account
///     name: example_connect
///     properties:
///       name: examplestorage2
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: BlobStorage
///   exampleManagedPrivateEndpoint:
///     type: azure:synapse:ManagedPrivateEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       targetResourceId: ${exampleConnect.id}
///       subresourceName: blob
///     options:
///       dependsOn:
///         - ${exampleFirewallRule}
/// ```
///
/// ## Import
///
/// Synapse Managed Private Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/managedPrivateEndpoint:ManagedPrivateEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/managedVirtualNetworks/default/managedPrivateEndpoints/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_private_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPrivateEndpointArgs {
        /// Specifies the name which should be used for this Managed Private Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the sub resource name which the Synapse Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Possible values are listed in [documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-overview#dns-configuration).
        #[builder(into)]
        pub subresource_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Synapse Workspace on which to create the Managed Private Endpoint. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A Synapse firewall rule including local IP is needed for managing current resource.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Private Link Enabled Remote Resource which this Synapse Private Endpoint should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedPrivateEndpointResult {
        /// Specifies the name which should be used for this Managed Private Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the sub resource name which the Synapse Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Possible values are listed in [documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-overview#dns-configuration).
        pub subresource_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Synapse Workspace on which to create the Managed Private Endpoint. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A Synapse firewall rule including local IP is needed for managing current resource.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Private Link Enabled Remote Resource which this Synapse Private Endpoint should be connected to. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedPrivateEndpointArgs,
    ) -> ManagedPrivateEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let subresource_name_binding = args.subresource_name.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let target_resource_id_binding = args.target_resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/managedPrivateEndpoint:ManagedPrivateEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subresourceName".into(),
                    value: &subresource_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedPrivateEndpointResult {
            name: o.get_field("name"),
            subresource_name: o.get_field("subresourceName"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
            target_resource_id: o.get_field("targetResourceId"),
        }
    }
}
