/// Manages a Backup Policy to back up PostgreSQL Flexible Server.
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
///   exampleBackupVault:
///     type: azure:dataprotection:BackupVault
///     name: example
///     properties:
///       name: example-backup-vault
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datastoreType: VaultStore
///       redundancy: LocallyRedundant
///       identity:
///         type: SystemAssigned
///   exampleBackupPolicyPostgresqlFlexibleServer:
///     type: azure:dataprotection:BackupPolicyPostgresqlFlexibleServer
///     name: example
///     properties:
///       name: example-backup-policy
///       vaultId: ${exampleBackupVault.id}
///       backupRepeatingTimeIntervals:
///         - R/2021-05-23T02:30:00+00:00/P1W
///       timeZone: India Standard Time
///       defaultRetentionRule:
///         lifeCycles:
///           - duration: P4M
///             dataStoreType: VaultStore
///       retentionRules:
///         - name: weekly
///           lifeCycles:
///             - duration: P6M
///               dataStoreType: VaultStore
///           priority: 20
///           criteria:
///             absoluteCriteria: FirstOfWeek
///         - name: thursday
///           lifeCycles:
///             - duration: P1W
///               dataStoreType: VaultStore
///           priority: 25
///           criteria:
///             daysOfWeeks:
///               - Thursday
///             scheduledBackupTimes:
///               - 2021-05-23T02:30:00Z
///         - name: monthly
///           lifeCycles:
///             - duration: P1D
///               dataStoreType: VaultStore
///           priority: 15
///           criteria:
///             weeksOfMonths:
///               - First
///               - Last
///             daysOfWeeks:
///               - Tuesday
///             scheduledBackupTimes:
///               - 2021-05-23T02:30:00Z
/// ```
///
/// ## Import
///
/// Backup Policy PostgreSQL Flexible Server's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupPolicyPostgresqlFlexibleServer:BackupPolicyPostgresqlFlexibleServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupPolicies/backupPolicy1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod backup_policy_postgresql_flexible_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlFlexibleServerArgs {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval format. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backup_repeating_time_intervals: pulumi_gestalt_rust::Input<
            Vec<String>,
        >,
        /// A `default_retention_rule` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub default_retention_rule: pulumi_gestalt_rust::Input<
            super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerDefaultRetentionRule,
        >,
        /// Specifies the name of the Backup Policy for the PostgreSQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub retention_rules: pulumi_gestalt_rust::Input<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the Backup Vault where the Backup Policy PostgreSQL Flexible Server should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vault_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlFlexibleServerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval format. Changing this forces a new resource to be created.
        pub backup_repeating_time_intervals: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `default_retention_rule` block as defined below. Changing this forces a new resource to be created.
        pub default_retention_rule: pulumi_gestalt_rust::Output<
            super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerDefaultRetentionRule,
        >,
        /// Specifies the name of the Backup Policy for the PostgreSQL Flexible Server. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new resource to be created.
        pub retention_rules: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new resource to be created.
        pub time_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Backup Vault where the Backup Policy PostgreSQL Flexible Server should exist. Changing this forces a new resource to be created.
        pub vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyPostgresqlFlexibleServerArgs,
    ) -> BackupPolicyPostgresqlFlexibleServerResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyPostgresqlFlexibleServerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> BackupPolicyPostgresqlFlexibleServerResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyPostgresqlFlexibleServerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> BackupPolicyPostgresqlFlexibleServerResult {
        let backup_repeating_time_intervals_binding = args
            .backup_repeating_time_intervals
            .get_output(ctx);
        let default_retention_rule_binding = args.default_retention_rule.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let retention_rules_binding = args.retention_rules.get_output(ctx);
        let time_zone_binding = args.time_zone.get_output(ctx);
        let vault_id_binding = args.vault_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyPostgresqlFlexibleServer:BackupPolicyPostgresqlFlexibleServer"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRepeatingTimeIntervals".into(),
                    value: &backup_repeating_time_intervals_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRetentionRule".into(),
                    value: &default_retention_rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
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
                    name: "vaultId".into(),
                    value: &vault_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        BackupPolicyPostgresqlFlexibleServerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            backup_repeating_time_intervals: o.get_field("backupRepeatingTimeIntervals"),
            default_retention_rule: o.get_field("defaultRetentionRule"),
            name: o.get_field("name"),
            retention_rules: o.get_field("retentionRules"),
            time_zone: o.get_field("timeZone"),
            vault_id: o.get_field("vaultId"),
        }
    }
}
