/// Manages a VMWare Replication Policy.
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
///         ResourceGroupArgs::builder().location("eastus").name("example-rg").build_struct(),
///     );
///     let exampleVMWareReplicationPolicy = vm_ware_replication_policy::create(
///         "exampleVMWareReplicationPolicy",
///         VmWareReplicationPolicyArgs::builder()
///             .application_consistent_snapshot_frequency_in_minutes(240)
///             .name("example-policy")
///             .recovery_point_retention_in_minutes(1440)
///             .recovery_vault_id("${exampleVault.id}")
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder()
///             .classic_vmware_replication_enabled(true)
///             .location("${example.location}")
///             .name("example-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .soft_delete_enabled(false)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VMWare Replication Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/vMWareReplicationPolicy:VMWareReplicationPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/vault1/providers/Microsoft.RecoveryServices/vaults/vault1/replicationPolicies/policy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vm_ware_replication_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VMWareReplicationPolicyArgs {
        /// Specifies the frequency at which to create application consistent recovery points. Must between `0` to `720`.
        #[builder(into)]
        pub application_consistent_snapshot_frequency_in_minutes: pulumi_gestalt_rust::InputOrOutput<
            i32,
        >,
        /// The name which should be used for this Classic Replication Policy. Changing this forces a new Replication Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the period up to which the recovery points will be retained. Must between `0` to `21600`.
        #[builder(into)]
        pub recovery_point_retention_in_minutes: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// ID of the Recovery Services Vault. Changing this forces a new Replication Policy to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VMWareReplicationPolicyResult {
        /// Specifies the frequency at which to create application consistent recovery points. Must between `0` to `720`.
        pub application_consistent_snapshot_frequency_in_minutes: pulumi_gestalt_rust::Output<
            i32,
        >,
        /// The name which should be used for this Classic Replication Policy. Changing this forces a new Replication Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the period up to which the recovery points will be retained. Must between `0` to `21600`.
        pub recovery_point_retention_in_minutes: pulumi_gestalt_rust::Output<i32>,
        /// ID of the Recovery Services Vault. Changing this forces a new Replication Policy to be created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VMWareReplicationPolicyArgs,
    ) -> VMWareReplicationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_consistent_snapshot_frequency_in_minutes_binding = args
            .application_consistent_snapshot_frequency_in_minutes
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let recovery_point_retention_in_minutes_binding = args
            .recovery_point_retention_in_minutes
            .get_output(context);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/vMWareReplicationPolicy:VMWareReplicationPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationConsistentSnapshotFrequencyInMinutes".into(),
                    value: &application_consistent_snapshot_frequency_in_minutes_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryPointRetentionInMinutes".into(),
                    value: &recovery_point_retention_in_minutes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VMWareReplicationPolicyResult {
            application_consistent_snapshot_frequency_in_minutes: o
                .get_field("applicationConsistentSnapshotFrequencyInMinutes"),
            name: o.get_field("name"),
            recovery_point_retention_in_minutes: o
                .get_field("recoveryPointRetentionInMinutes"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
        }
    }
}
