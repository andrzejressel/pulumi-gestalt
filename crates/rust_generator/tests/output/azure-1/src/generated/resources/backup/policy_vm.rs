/// Manages an Azure Backup VM Backup Policy.
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
///             .name("tfex-recovery_vault")
///             .build_struct(),
///     );
///     let examplePolicyVM = policy_vm::create(
///         "examplePolicyVM",
///         PolicyVmArgs::builder()
///             .backup(
///                 PolicyVmBackup::builder().frequency("Daily").time("23:00").build_struct(),
///             )
///             .name("tfex-recovery-vault-policy")
///             .recovery_vault_name("${exampleVault.name}")
///             .resource_group_name("${example.name}")
///             .retention_daily(PolicyVmRetentionDaily::builder().count(10).build_struct())
///             .retention_monthly(
///                 PolicyVmRetentionMonthly::builder()
///                     .count(7)
///                     .weekdays(vec!["Sunday", "Wednesday",])
///                     .weeks(vec!["First", "Last",])
///                     .build_struct(),
///             )
///             .retention_weekly(
///                 PolicyVmRetentionWeekly::builder()
///                     .count(42)
///                     .weekdays(vec!["Sunday", "Wednesday", "Friday", "Saturday",])
///                     .build_struct(),
///             )
///             .retention_yearly(
///                 PolicyVmRetentionYearly::builder()
///                     .count(77)
///                     .months(vec!["January",])
///                     .weekdays(vec!["Sunday",])
///                     .weeks(vec!["Last",])
///                     .build_struct(),
///             )
///             .timezone("UTC")
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("tfex-recovery-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VM Backup Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:backup/policyVM:PolicyVM policy1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/example-recovery-vault/backupPolicies/policy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_vm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyVMArgs {
        /// Configures the Policy backup frequency, times & days as documented in the `backup` block below.
        #[builder(into)]
        pub backup: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::backup::PolicyVmBackup,
        >,
        /// Specifies the instant restore resource group name as documented in the `instant_restore_resource_group` block below.
        #[builder(into, default)]
        pub instant_restore_resource_group: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::PolicyVmInstantRestoreResourceGroup>,
        >,
        /// Specifies the instant restore retention range in days. Possible values are between `1` and `5` when `policy_type` is `V1`, and `1` to `30` when `policy_type` is `V2`.
        ///
        /// > **NOTE:** `instant_restore_retention_days` **must** be set to `5` if the backup frequency is set to `Weekly`.
        #[builder(into, default)]
        pub instant_restore_retention_days: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Specifies the name of the Backup Policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of the Backup Policy. Possible values are `V1` and `V2` where `V2` stands for the Enhanced Policy. Defaults to `V1`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub policy_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configures the policy daily retention as documented in the `retention_daily` block below. Required when backup frequency is `Daily`.
        #[builder(into, default)]
        pub retention_daily: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::PolicyVmRetentionDaily>,
        >,
        /// Configures the policy monthly retention as documented in the `retention_monthly` block below.
        #[builder(into, default)]
        pub retention_monthly: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::PolicyVmRetentionMonthly>,
        >,
        /// Configures the policy weekly retention as documented in the `retention_weekly` block below. Required when backup frequency is `Weekly`.
        #[builder(into, default)]
        pub retention_weekly: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::PolicyVmRetentionWeekly>,
        >,
        /// Configures the policy yearly retention as documented in the `retention_yearly` block below.
        #[builder(into, default)]
        pub retention_yearly: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::PolicyVmRetentionYearly>,
        >,
        /// A `tiering_policy` block as defined below.
        #[builder(into, default)]
        pub tiering_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::PolicyVmTieringPolicy>,
        >,
        /// Specifies the timezone. [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). Defaults to `UTC`
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PolicyVMResult {
        /// Configures the Policy backup frequency, times & days as documented in the `backup` block below.
        pub backup: pulumi_gestalt_rust::Output<
            super::super::types::backup::PolicyVmBackup,
        >,
        /// Specifies the instant restore resource group name as documented in the `instant_restore_resource_group` block below.
        pub instant_restore_resource_group: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::PolicyVmInstantRestoreResourceGroup>,
        >,
        /// Specifies the instant restore retention range in days. Possible values are between `1` and `5` when `policy_type` is `V1`, and `1` to `30` when `policy_type` is `V2`.
        ///
        /// > **NOTE:** `instant_restore_retention_days` **must** be set to `5` if the backup frequency is set to `Weekly`.
        pub instant_restore_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the Backup Policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Type of the Backup Policy. Possible values are `V1` and `V2` where `V2` stands for the Enhanced Policy. Defaults to `V1`. Changing this forces a new resource to be created.
        pub policy_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the policy. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Configures the policy daily retention as documented in the `retention_daily` block below. Required when backup frequency is `Daily`.
        pub retention_daily: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::PolicyVmRetentionDaily>,
        >,
        /// Configures the policy monthly retention as documented in the `retention_monthly` block below.
        pub retention_monthly: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::PolicyVmRetentionMonthly>,
        >,
        /// Configures the policy weekly retention as documented in the `retention_weekly` block below. Required when backup frequency is `Weekly`.
        pub retention_weekly: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::PolicyVmRetentionWeekly>,
        >,
        /// Configures the policy yearly retention as documented in the `retention_yearly` block below.
        pub retention_yearly: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::PolicyVmRetentionYearly>,
        >,
        /// A `tiering_policy` block as defined below.
        pub tiering_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::PolicyVmTieringPolicy>,
        >,
        /// Specifies the timezone. [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). Defaults to `UTC`
        pub timezone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyVMArgs,
    ) -> PolicyVMResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_binding = args.backup.get_output(context);
        let instant_restore_resource_group_binding = args
            .instant_restore_resource_group
            .get_output(context);
        let instant_restore_retention_days_binding = args
            .instant_restore_retention_days
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_type_binding = args.policy_type.get_output(context);
        let recovery_vault_name_binding = args.recovery_vault_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retention_daily_binding = args.retention_daily.get_output(context);
        let retention_monthly_binding = args.retention_monthly.get_output(context);
        let retention_weekly_binding = args.retention_weekly.get_output(context);
        let retention_yearly_binding = args.retention_yearly.get_output(context);
        let tiering_policy_binding = args.tiering_policy.get_output(context);
        let timezone_binding = args.timezone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:backup/policyVM:PolicyVM".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backup".into(),
                    value: &backup_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instantRestoreResourceGroup".into(),
                    value: &instant_restore_resource_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instantRestoreRetentionDays".into(),
                    value: &instant_restore_retention_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding.drop_type(),
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
                    name: "retentionDaily".into(),
                    value: &retention_daily_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionMonthly".into(),
                    value: &retention_monthly_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionWeekly".into(),
                    value: &retention_weekly_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionYearly".into(),
                    value: &retention_yearly_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tieringPolicy".into(),
                    value: &tiering_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyVMResult {
            backup: o.get_field("backup"),
            instant_restore_resource_group: o.get_field("instantRestoreResourceGroup"),
            instant_restore_retention_days: o.get_field("instantRestoreRetentionDays"),
            name: o.get_field("name"),
            policy_type: o.get_field("policyType"),
            recovery_vault_name: o.get_field("recoveryVaultName"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_daily: o.get_field("retentionDaily"),
            retention_monthly: o.get_field("retentionMonthly"),
            retention_weekly: o.get_field("retentionWeekly"),
            retention_yearly: o.get_field("retentionYearly"),
            tiering_policy: o.get_field("tieringPolicy"),
            timezone: o.get_field("timezone"),
        }
    }
}
