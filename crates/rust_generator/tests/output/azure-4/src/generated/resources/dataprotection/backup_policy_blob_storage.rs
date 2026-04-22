/// Manages a Backup Policy Blob Storage.
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
///     let exampleBackupPolicyBlobStorage = backup_policy_blob_storage::create(
///         "exampleBackupPolicyBlobStorage",
///         BackupPolicyBlobStorageArgs::builder()
///             .name("example-backup-policy")
///             .operational_default_retention_duration("P30D")
///             .vault_id("${exampleBackupVault.id}")
///             .build_struct(),
///     );
///     let exampleBackupVault = backup_vault::create(
///         "exampleBackupVault",
///         BackupVaultArgs::builder()
///             .datastore_type("VaultStore")
///             .location("${example.location}")
///             .name("example-backup-vault")
///             .redundancy("LocallyRedundant")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Backup Policy Blob Storages can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupPolicyBlobStorage:BackupPolicyBlobStorage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupPolicies/backupPolicy1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod backup_policy_blob_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyBlobStorageArgs {
        /// Specifies a list of repeating time interval. It should follow `ISO 8601` repeating time interval. Changing this forces a new Backup Policy Blob Storage to be created.
        #[builder(into, default)]
        pub backup_repeating_time_intervals: pulumi_gestalt_rust::Input<
            Option<Vec<String>>,
        >,
        /// The name which should be used for this Backup Policy Blob Storage. Changing this forces a new Backup Policy Blob Storage to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The duration of operational default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy Blob Storage to be created.
        #[builder(into, default)]
        pub operational_default_retention_duration: pulumi_gestalt_rust::Input<
            Option<String>,
        >,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy Blob Storage to be created.
        ///
        /// > **Note:** Setting `retention_rule` also requires setting `vault_default_retention_duration`.
        #[builder(into, default)]
        pub retention_rules: pulumi_gestalt_rust::Input<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyBlobStorageRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy Blob Storage to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::Input<Option<String>>,
        /// The duration of vault default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy Blob Storage to be created.
        ///
        /// > **Note:** Setting `vault_default_retention_duration` also requires setting `backup_repeating_time_intervals`. At least one of `operational_default_retention_duration` or `vault_default_retention_duration` must be specified.
        #[builder(into, default)]
        pub vault_default_retention_duration: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the Backup Vault within which the Backup Policy Blob Storage should exist. Changing this forces a new Backup Policy Blob Storage to be created.
        #[builder(into)]
        pub vault_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyBlobStorageResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of repeating time interval. It should follow `ISO 8601` repeating time interval. Changing this forces a new Backup Policy Blob Storage to be created.
        pub backup_repeating_time_intervals: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The name which should be used for this Backup Policy Blob Storage. Changing this forces a new Backup Policy Blob Storage to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The duration of operational default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy Blob Storage to be created.
        pub operational_default_retention_duration: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy Blob Storage to be created.
        ///
        /// > **Note:** Setting `retention_rule` also requires setting `vault_default_retention_duration`.
        pub retention_rules: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyBlobStorageRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy Blob Storage to be created.
        pub time_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The duration of vault default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy Blob Storage to be created.
        ///
        /// > **Note:** Setting `vault_default_retention_duration` also requires setting `backup_repeating_time_intervals`. At least one of `operational_default_retention_duration` or `vault_default_retention_duration` must be specified.
        pub vault_default_retention_duration: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the Backup Vault within which the Backup Policy Blob Storage should exist. Changing this forces a new Backup Policy Blob Storage to be created.
        pub vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyBlobStorageArgs,
    ) -> BackupPolicyBlobStorageResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyBlobStorageArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> BackupPolicyBlobStorageResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyBlobStorageArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> BackupPolicyBlobStorageResult {
        let backup_repeating_time_intervals_binding = args
            .backup_repeating_time_intervals
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let operational_default_retention_duration_binding = args
            .operational_default_retention_duration
            .get_output(ctx);
        let retention_rules_binding = args.retention_rules.get_output(ctx);
        let time_zone_binding = args.time_zone.get_output(ctx);
        let vault_default_retention_duration_binding = args
            .vault_default_retention_duration
            .get_output(ctx);
        let vault_id_binding = args.vault_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyBlobStorage:BackupPolicyBlobStorage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRepeatingTimeIntervals".into(),
                    value: &backup_repeating_time_intervals_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationalDefaultRetentionDuration".into(),
                    value: &operational_default_retention_duration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionRules".into(),
                    value: &retention_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultDefaultRetentionDuration".into(),
                    value: &vault_default_retention_duration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultId".into(),
                    value: &vault_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        BackupPolicyBlobStorageResult {
            id: o.get_id(),
            urn: o.get_urn(),
            backup_repeating_time_intervals: o.get_field("backupRepeatingTimeIntervals"),
            name: o.get_field("name"),
            operational_default_retention_duration: o
                .get_field("operationalDefaultRetentionDuration"),
            retention_rules: o.get_field("retentionRules"),
            time_zone: o.get_field("timeZone"),
            vault_default_retention_duration: o
                .get_field("vaultDefaultRetentionDuration"),
            vault_id: o.get_field("vaultId"),
        }
    }
}
