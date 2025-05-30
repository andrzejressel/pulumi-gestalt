/// Manages a HyperV site recovery network mapping on Azure. A HyperV network mapping decides how to translate connected networks when a VM is migrated from HyperV VMM Center to Azure.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   target:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-network-mapping
///       location: East US
///   vault:
///     type: azure:recoveryservices:Vault
///     properties:
///       name: example-recovery-vault
///       location: ${target.location}
///       resourceGroupName: ${target.name}
///       sku: Standard
///   targetVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: target
///     properties:
///       name: network
///       resourceGroupName: ${target.name}
///       addressSpaces:
///         - 192.168.2.0/24
///       location: ${target.location}
///   recovery-mapping:
///     type: azure:siterecovery:HypervNetworkMapping
///     properties:
///       name: recovery-network-mapping
///       recoveryVaultId: ${vault.id}
///       sourceSystemCenterVirtualMachineManagerName: my-vmm-server
///       sourceNetworkName: my-vmm-network
///       targetNetworkId: ${targetVirtualNetwork.id}
/// ```
///
/// ## Import
///
/// Site Recovery Network Mapping can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/hypervNetworkMapping:HypervNetworkMapping azurerm_site_recovery_hyperv_network_mapping.mymapping /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/primary-fabric-name/replicationNetworks/azureNetwork/replicationNetworkMappings/mapping-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hyperv_network_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HypervNetworkMappingArgs {
        /// The name of the HyperV network mapping. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Recovery Services Vault where the HyperV network mapping should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the primary network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_network_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of source System Center Virtual Machine Manager where the source network exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_system_center_virtual_machine_manager_name: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The id of the recovery network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HypervNetworkMappingResult {
        /// The name of the HyperV network mapping. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Recovery Services Vault where the HyperV network mapping should be created. Changing this forces a new resource to be created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The Name of the primary network. Changing this forces a new resource to be created.
        pub source_network_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of source System Center Virtual Machine Manager where the source network exists. Changing this forces a new resource to be created.
        pub source_system_center_virtual_machine_manager_name: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The id of the recovery network. Changing this forces a new resource to be created.
        pub target_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HypervNetworkMappingArgs,
    ) -> HypervNetworkMappingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(context);
        let source_network_name_binding = args.source_network_name.get_output(context);
        let source_system_center_virtual_machine_manager_name_binding = args
            .source_system_center_virtual_machine_manager_name
            .get_output(context);
        let target_network_id_binding = args.target_network_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/hypervNetworkMapping:HypervNetworkMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceNetworkName".into(),
                    value: &source_network_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceSystemCenterVirtualMachineManagerName".into(),
                    value: &source_system_center_virtual_machine_manager_name_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetNetworkId".into(),
                    value: &target_network_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HypervNetworkMappingResult {
            name: o.get_field("name"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
            source_network_name: o.get_field("sourceNetworkName"),
            source_system_center_virtual_machine_manager_name: o
                .get_field("sourceSystemCenterVirtualMachineManagerName"),
            target_network_id: o.get_field("targetNetworkId"),
        }
    }
}
