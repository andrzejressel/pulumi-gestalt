/// Provides an AWS Backup vault notifications resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testTopic:
///     type: aws:sns:Topic
///     name: test
///     properties:
///       name: backup-vault-events
///   testTopicPolicy:
///     type: aws:sns:TopicPolicy
///     name: test
///     properties:
///       arn: ${testTopic.arn}
///       policy: ${test.json}
///   testVaultNotifications:
///     type: aws:backup:VaultNotifications
///     name: test
///     properties:
///       backupVaultName: example_backup_vault
///       snsTopicArn: ${testTopic.arn}
///       backupVaultEvents:
///         - BACKUP_JOB_STARTED
///         - RESTORE_JOB_COMPLETED
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         policyId: __default_policy_ID
///         statements:
///           - actions:
///               - SNS:Publish
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - backup.amazonaws.com
///             resources:
///               - ${testTopic.arn}
///             sid: __default_statement_ID
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup vault notifications using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/vaultNotifications:VaultNotifications test TestVault
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vault_notifications {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultNotificationsArgs {
        /// An array of events that indicate the status of jobs to back up resources to the backup vault.
        #[builder(into)]
        pub backup_vault_events: pulumi_gestalt_rust::Input<Vec<String>>,
        /// Name of the backup vault to add notifications for.
        #[builder(into)]
        pub backup_vault_name: pulumi_gestalt_rust::Input<String>,
        /// The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events
        #[builder(into)]
        pub sns_topic_arn: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct VaultNotificationsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the vault.
        pub backup_vault_arn: pulumi_gestalt_rust::Output<String>,
        /// An array of events that indicate the status of jobs to back up resources to the backup vault.
        pub backup_vault_events: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the backup vault to add notifications for.
        pub backup_vault_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events
        pub sns_topic_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultNotificationsArgs,
    ) -> VaultNotificationsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultNotificationsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VaultNotificationsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultNotificationsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VaultNotificationsResult {
        let backup_vault_events_binding = args.backup_vault_events.get_output(ctx);
        let backup_vault_name_binding = args.backup_vault_name.get_output(ctx);
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/vaultNotifications:VaultNotifications".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVaultEvents".into(),
                    value: &backup_vault_events_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVaultName".into(),
                    value: &backup_vault_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsTopicArn".into(),
                    value: &sns_topic_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VaultNotificationsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            backup_vault_arn: o.get_field("backupVaultArn"),
            backup_vault_events: o.get_field("backupVaultEvents"),
            backup_vault_name: o.get_field("backupVaultName"),
            sns_topic_arn: o.get_field("snsTopicArn"),
        }
    }
}
