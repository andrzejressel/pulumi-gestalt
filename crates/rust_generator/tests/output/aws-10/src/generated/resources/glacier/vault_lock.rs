/// ## Example Usage
///
/// ### Testing Glacier Vault Lock Policy
///
/// ```yaml
/// resources:
///   exampleVault:
///     type: aws:glacier:Vault
///     name: example
///     properties:
///       name: example
///   exampleVaultLock:
///     type: aws:glacier:VaultLock
///     name: example
///     properties:
///       completeLock: false
///       policy: ${example.json}
///       vaultName: ${exampleVault.name}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - glacier:DeleteArchive
///             effect: Deny
///             resources:
///               - ${exampleVault.arn}
///             conditions:
///               - test: NumericLessThanEquals
///                 variable: glacier:ArchiveAgeinDays
///                 values:
///                   - '365'
/// ```
///
/// ### Permanently Applying Glacier Vault Lock Policy
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vault_lock::create(
///         "example",
///         VaultLockArgs::builder()
///             .complete_lock(true)
///             .policy("${exampleAwsIamPolicyDocument.json}")
///             .vault_name("${exampleAwsGlacierVault.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glacier Vault Locks using the Glacier Vault name. For example:
///
/// ```sh
/// $ pulumi import aws:glacier/vaultLock:VaultLock example example-vault
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vault_lock {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultLockArgs {
        /// Boolean whether to permanently apply this Glacier Lock Policy. Once completed, this cannot be undone. If set to `false`, the Glacier Lock Policy remains in a testing mode for 24 hours. After that time, the Glacier Lock Policy is automatically removed by Glacier and the this provider resource will show as needing recreation. Changing this from `false` to `true` will show as resource recreation, which is expected. Changing this from `true` to `false` is not possible unless the Glacier Vault is recreated at the same time.
        #[builder(into)]
        pub complete_lock: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Allow this provider to ignore the error returned when attempting to delete the Glacier Lock Policy. This can be used to delete or recreate the Glacier Vault via this provider, for example, if the Glacier Vault Lock policy permits that action. This should only be used in conjunction with `complete_lock` being set to `true`.
        #[builder(into, default)]
        pub ignore_deletion_error: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// JSON string containing the IAM policy to apply as the Glacier Vault Lock policy.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Glacier Vault.
        #[builder(into)]
        pub vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VaultLockResult {
        /// Boolean whether to permanently apply this Glacier Lock Policy. Once completed, this cannot be undone. If set to `false`, the Glacier Lock Policy remains in a testing mode for 24 hours. After that time, the Glacier Lock Policy is automatically removed by Glacier and the this provider resource will show as needing recreation. Changing this from `false` to `true` will show as resource recreation, which is expected. Changing this from `true` to `false` is not possible unless the Glacier Vault is recreated at the same time.
        pub complete_lock: pulumi_gestalt_rust::Output<bool>,
        /// Allow this provider to ignore the error returned when attempting to delete the Glacier Lock Policy. This can be used to delete or recreate the Glacier Vault via this provider, for example, if the Glacier Vault Lock policy permits that action. This should only be used in conjunction with `complete_lock` being set to `true`.
        pub ignore_deletion_error: pulumi_gestalt_rust::Output<Option<bool>>,
        /// JSON string containing the IAM policy to apply as the Glacier Vault Lock policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The name of the Glacier Vault.
        pub vault_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultLockArgs,
    ) -> VaultLockResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let complete_lock_binding = args.complete_lock.get_output(context);
        let ignore_deletion_error_binding = args
            .ignore_deletion_error
            .get_output(context);
        let policy_binding = args.policy.get_output(context);
        let vault_name_binding = args.vault_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glacier/vaultLock:VaultLock".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "completeLock".into(),
                    value: &complete_lock_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreDeletionError".into(),
                    value: &ignore_deletion_error_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultName".into(),
                    value: &vault_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VaultLockResult {
            complete_lock: o.get_field("completeLock"),
            ignore_deletion_error: o.get_field("ignoreDeletionError"),
            policy: o.get_field("policy"),
            vault_name: o.get_field("vaultName"),
        }
    }
}
