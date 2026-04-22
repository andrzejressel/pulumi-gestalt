/// Manages Azure Backup for an Azure VM
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: tfex-recovery_vault
///       location: West Europe
///   exampleVault:
///     type: azure:recoveryservices:Vault
///     name: example
///     properties:
///       name: tfex-recovery-vault
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Standard
///   examplePolicyVM:
///     type: azure:backup:PolicyVM
///     name: example
///     properties:
///       name: tfex-recovery-vault-policy
///       resourceGroupName: ${exampleResourceGroup.name}
///       recoveryVaultName: ${exampleVault.name}
///       backup:
///         frequency: Daily
///         time: 23:00
///       retentionDaily:
///         count: 10
///   vm1:
///     type: azure:backup:ProtectedVM
///     properties:
///       resourceGroupName: ${exampleResourceGroup.name}
///       recoveryVaultName: ${exampleVault.name}
///       sourceVmId: ${example.id}
///       backupPolicyId: ${examplePolicyVM.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:compute:getVirtualMachine
///       arguments:
///         name: example-vm
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Recovery Services Protected VMs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:backup/protectedVM:ProtectedVM item1 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/example-recovery-vault/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;group1;vm1/protectedItems/vm;iaasvmcontainerv2;group1;vm1"
/// ```
///
/// Note the ID requires quoting as there are semicolons
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod protected_vm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectedVMArgs {
        /// Specifies the id of the backup policy to use. Required in creation or when `protection_stopped` is not specified.
        #[builder(into, default)]
        pub backup_policy_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be excluded for VM Protection.
        #[builder(into, default)]
        pub exclude_disk_luns: pulumi_gestalt_rust::Input<Option<Vec<i32>>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be included for VM Protection.
        #[builder(into, default)]
        pub include_disk_luns: pulumi_gestalt_rust::Input<Option<Vec<i32>>>,
        /// Specifies Protection state of the backup. Possible values are `Invalid`, `IRPending`, `Protected`, `ProtectionStopped`, `ProtectionError` and `ProtectionPaused`.
        #[builder(into, default)]
        pub protection_state: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name of the Resource Group **associated with** the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the ID of the VM to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** After creation, the `source_vm_id` property can be removed without forcing a new resource to be created; however, setting it to a different ID will create a new resource.
        /// This allows the source vm to be deleted without having to remove the backup.
        #[builder(into, default)]
        pub source_vm_id: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProtectedVMResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the id of the backup policy to use. Required in creation or when `protection_stopped` is not specified.
        pub backup_policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be excluded for VM Protection.
        pub exclude_disk_luns: pulumi_gestalt_rust::Output<Option<Vec<i32>>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be included for VM Protection.
        pub include_disk_luns: pulumi_gestalt_rust::Output<Option<Vec<i32>>>,
        /// Specifies Protection state of the backup. Possible values are `Invalid`, `IRPending`, `Protected`, `ProtectionStopped`, `ProtectionError` and `ProtectionPaused`.
        pub protection_state: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group **associated with** the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the VM to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** After creation, the `source_vm_id` property can be removed without forcing a new resource to be created; however, setting it to a different ID will create a new resource.
        /// This allows the source vm to be deleted without having to remove the backup.
        pub source_vm_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectedVMArgs,
    ) -> ProtectedVMResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectedVMArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ProtectedVMResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectedVMArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ProtectedVMResult {
        let backup_policy_id_binding = args.backup_policy_id.get_output(ctx);
        let exclude_disk_luns_binding = args.exclude_disk_luns.get_output(ctx);
        let include_disk_luns_binding = args.include_disk_luns.get_output(ctx);
        let protection_state_binding = args.protection_state.get_output(ctx);
        let recovery_vault_name_binding = args.recovery_vault_name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let source_vm_id_binding = args.source_vm_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:backup/protectedVM:ProtectedVM".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeDiskLuns".into(),
                    value: &exclude_disk_luns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeDiskLuns".into(),
                    value: &include_disk_luns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectionState".into(),
                    value: &protection_state_binding.drop_type(),
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
                    name: "sourceVmId".into(),
                    value: &source_vm_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ProtectedVMResult {
            id: o.get_id(),
            urn: o.get_urn(),
            backup_policy_id: o.get_field("backupPolicyId"),
            exclude_disk_luns: o.get_field("excludeDiskLuns"),
            include_disk_luns: o.get_field("includeDiskLuns"),
            protection_state: o.get_field("protectionState"),
            recovery_vault_name: o.get_field("recoveryVaultName"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_vm_id: o.get_field("sourceVmId"),
        }
    }
}
