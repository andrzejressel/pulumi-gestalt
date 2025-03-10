/// Manages a VM replicated using Azure Site Recovery (Azure to Azure only). A replicated VM keeps a copiously updated image of the VM in another region in order to be able to start the VM in that region in case of a disaster.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Site Recovery Replicated VM's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/replicatedVM:ReplicatedVM vmreplication /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/fabric-name/replicationProtectionContainers/protection-container-name/replicationProtectedItems/vm-replication-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replicated_vm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicatedVMArgs {
        /// One or more `managed_disk` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::siterecovery::ReplicatedVmManagedDisk>>,
        >,
        /// Name of group in which all machines will replicate together and have shared crash consistent and app-consistent recovery points when failed over.
        #[builder(into, default)]
        pub multi_vm_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the replication for the replicated VM. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `network_interface` block as defined below.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::siterecovery::ReplicatedVmNetworkInterface>>,
        >,
        /// Id of the policy to use for this replicated vm. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_replication_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of fabric that should contain this replication. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_recovery_fabric_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the protection container to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_recovery_protection_container_name: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Id of the VM to replicate Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_vm_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of availability set that the new VM should belong to when a failover is done.
        #[builder(into, default)]
        pub target_availability_set_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Id of the storage account which the new VM should used for boot diagnostic when a failover is done.
        #[builder(into, default)]
        pub target_boot_diagnostic_storage_account_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Id of the Capacity reservation group where the new VM should belong to when a failover is done.
        #[builder(into, default)]
        pub target_capacity_reservation_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the Edge Zone within the Azure Region where this Managed Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub target_edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network to use when a failover is done (recommended to set if any network_interface is configured for failover).
        #[builder(into, default)]
        pub target_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Id of Proximity Placement Group the new VM should belong to when a failover is done.
        #[builder(into, default)]
        pub target_proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Id of fabric where the VM replication should be handled when a failover is done. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_recovery_fabric_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of protection container where the VM replication should be created when a failover is done. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_recovery_protection_container_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Id of resource group where the VM should be created when a failover is done. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the Virtual Machine Scale Set which the new Vm should belong to when a failover is done.
        #[builder(into, default)]
        pub target_virtual_machine_scale_set_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the size the Virtual Machine should have.
        #[builder(into, default)]
        pub target_virtual_machine_size: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the Availability Zone where the Failover VM should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub target_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network to use when a test failover is done.
        #[builder(into, default)]
        pub test_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `unmanaged_disk` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub unmanaged_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::siterecovery::ReplicatedVmUnmanagedDisk>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicatedVMResult {
        /// One or more `managed_disk` block as defined below. Changing this forces a new resource to be created.
        pub managed_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::siterecovery::ReplicatedVmManagedDisk>,
        >,
        /// Name of group in which all machines will replicate together and have shared crash consistent and app-consistent recovery points when failed over.
        pub multi_vm_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the replication for the replicated VM. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_interface` block as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::siterecovery::ReplicatedVmNetworkInterface>,
        >,
        /// Id of the policy to use for this replicated vm. Changing this forces a new resource to be created.
        pub recovery_replication_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Name of fabric that should contain this replication. Changing this forces a new resource to be created.
        pub source_recovery_fabric_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the protection container to use. Changing this forces a new resource to be created.
        pub source_recovery_protection_container_name: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Id of the VM to replicate Changing this forces a new resource to be created.
        pub source_vm_id: pulumi_gestalt_rust::Output<String>,
        /// Id of availability set that the new VM should belong to when a failover is done.
        pub target_availability_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Id of the storage account which the new VM should used for boot diagnostic when a failover is done.
        pub target_boot_diagnostic_storage_account_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Id of the Capacity reservation group where the new VM should belong to when a failover is done.
        pub target_capacity_reservation_group_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the Edge Zone within the Azure Region where this Managed Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        pub target_edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Network to use when a failover is done (recommended to set if any network_interface is configured for failover).
        pub target_network_id: pulumi_gestalt_rust::Output<String>,
        /// Id of Proximity Placement Group the new VM should belong to when a failover is done.
        pub target_proximity_placement_group_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Id of fabric where the VM replication should be handled when a failover is done. Changing this forces a new resource to be created.
        pub target_recovery_fabric_id: pulumi_gestalt_rust::Output<String>,
        /// Id of protection container where the VM replication should be created when a failover is done. Changing this forces a new resource to be created.
        pub target_recovery_protection_container_id: pulumi_gestalt_rust::Output<String>,
        /// Id of resource group where the VM should be created when a failover is done. Changing this forces a new resource to be created.
        pub target_resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// Id of the Virtual Machine Scale Set which the new Vm should belong to when a failover is done.
        pub target_virtual_machine_scale_set_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the size the Virtual Machine should have.
        pub target_virtual_machine_size: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Availability Zone where the Failover VM should exist. Changing this forces a new resource to be created.
        pub target_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Network to use when a test failover is done.
        pub test_network_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `unmanaged_disk` block as defined below. Changing this forces a new resource to be created.
        pub unmanaged_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::siterecovery::ReplicatedVmUnmanagedDisk>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicatedVMArgs,
    ) -> ReplicatedVMResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_disks_binding = args.managed_disks.get_output(context);
        let multi_vm_group_name_binding = args.multi_vm_group_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let recovery_replication_policy_id_binding = args
            .recovery_replication_policy_id
            .get_output(context);
        let recovery_vault_name_binding = args.recovery_vault_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let source_recovery_fabric_name_binding = args
            .source_recovery_fabric_name
            .get_output(context);
        let source_recovery_protection_container_name_binding = args
            .source_recovery_protection_container_name
            .get_output(context);
        let source_vm_id_binding = args.source_vm_id.get_output(context);
        let target_availability_set_id_binding = args
            .target_availability_set_id
            .get_output(context);
        let target_boot_diagnostic_storage_account_id_binding = args
            .target_boot_diagnostic_storage_account_id
            .get_output(context);
        let target_capacity_reservation_group_id_binding = args
            .target_capacity_reservation_group_id
            .get_output(context);
        let target_edge_zone_binding = args.target_edge_zone.get_output(context);
        let target_network_id_binding = args.target_network_id.get_output(context);
        let target_proximity_placement_group_id_binding = args
            .target_proximity_placement_group_id
            .get_output(context);
        let target_recovery_fabric_id_binding = args
            .target_recovery_fabric_id
            .get_output(context);
        let target_recovery_protection_container_id_binding = args
            .target_recovery_protection_container_id
            .get_output(context);
        let target_resource_group_id_binding = args
            .target_resource_group_id
            .get_output(context);
        let target_virtual_machine_scale_set_id_binding = args
            .target_virtual_machine_scale_set_id
            .get_output(context);
        let target_virtual_machine_size_binding = args
            .target_virtual_machine_size
            .get_output(context);
        let target_zone_binding = args.target_zone.get_output(context);
        let test_network_id_binding = args.test_network_id.get_output(context);
        let unmanaged_disks_binding = args.unmanaged_disks.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/replicatedVM:ReplicatedVM".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedDisks".into(),
                    value: &managed_disks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiVmGroupName".into(),
                    value: &multi_vm_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryReplicationPolicyId".into(),
                    value: &recovery_replication_policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultName".into(),
                    value: &recovery_vault_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRecoveryFabricName".into(),
                    value: &source_recovery_fabric_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRecoveryProtectionContainerName".into(),
                    value: &source_recovery_protection_container_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVmId".into(),
                    value: &source_vm_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetAvailabilitySetId".into(),
                    value: &target_availability_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetBootDiagnosticStorageAccountId".into(),
                    value: &target_boot_diagnostic_storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetCapacityReservationGroupId".into(),
                    value: &target_capacity_reservation_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetEdgeZone".into(),
                    value: &target_edge_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetNetworkId".into(),
                    value: &target_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetProximityPlacementGroupId".into(),
                    value: &target_proximity_placement_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRecoveryFabricId".into(),
                    value: &target_recovery_fabric_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRecoveryProtectionContainerId".into(),
                    value: &target_recovery_protection_container_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceGroupId".into(),
                    value: &target_resource_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVirtualMachineScaleSetId".into(),
                    value: &target_virtual_machine_scale_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVirtualMachineSize".into(),
                    value: &target_virtual_machine_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetZone".into(),
                    value: &target_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "testNetworkId".into(),
                    value: &test_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "unmanagedDisks".into(),
                    value: &unmanaged_disks_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicatedVMResult {
            managed_disks: o.get_field("managedDisks"),
            multi_vm_group_name: o.get_field("multiVmGroupName"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            recovery_replication_policy_id: o.get_field("recoveryReplicationPolicyId"),
            recovery_vault_name: o.get_field("recoveryVaultName"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_recovery_fabric_name: o.get_field("sourceRecoveryFabricName"),
            source_recovery_protection_container_name: o
                .get_field("sourceRecoveryProtectionContainerName"),
            source_vm_id: o.get_field("sourceVmId"),
            target_availability_set_id: o.get_field("targetAvailabilitySetId"),
            target_boot_diagnostic_storage_account_id: o
                .get_field("targetBootDiagnosticStorageAccountId"),
            target_capacity_reservation_group_id: o
                .get_field("targetCapacityReservationGroupId"),
            target_edge_zone: o.get_field("targetEdgeZone"),
            target_network_id: o.get_field("targetNetworkId"),
            target_proximity_placement_group_id: o
                .get_field("targetProximityPlacementGroupId"),
            target_recovery_fabric_id: o.get_field("targetRecoveryFabricId"),
            target_recovery_protection_container_id: o
                .get_field("targetRecoveryProtectionContainerId"),
            target_resource_group_id: o.get_field("targetResourceGroupId"),
            target_virtual_machine_scale_set_id: o
                .get_field("targetVirtualMachineScaleSetId"),
            target_virtual_machine_size: o.get_field("targetVirtualMachineSize"),
            target_zone: o.get_field("targetZone"),
            test_network_id: o.get_field("testNetworkId"),
            unmanaged_disks: o.get_field("unmanagedDisks"),
        }
    }
}
