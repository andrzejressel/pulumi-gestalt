/// Manages a Customer Managed Key for the Databricks Workspaces Root Databricks File System(DBFS)
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
///   exampleWorkspace:
///     type: azure:databricks:Workspace
///     name: example
///     properties:
///       name: databricks-test
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: premium
///       customerManagedKeyEnabled: true
///       tags:
///         Environment: Production
///   exampleWorkspaceRootDbfsCustomerManagedKey:
///     type: azure:databricks:WorkspaceRootDbfsCustomerManagedKey
///     name: example
///     properties:
///       workspaceId: ${exampleWorkspace.id}
///       keyVaultKeyId: ${exampleKey.id}
///     options:
///       dependsOn:
///         - ${databricks}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       purgeProtectionEnabled: true
///       softDeleteRetentionDays: 7
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: example-certificate
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///     options:
///       dependsOn:
///         - ${terraform}
///   terraform:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleKeyVault.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - Purge
///         - Recover
///         - Update
///         - List
///         - Decrypt
///         - Sign
///         - GetRotationPolicy
///   databricks:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleWorkspace.storageAccountIdentities[0].tenantId}
///       objectId: ${exampleWorkspace.storageAccountIdentities[0].principalId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - Purge
///         - Recover
///         - Update
///         - List
///         - Decrypt
///         - Sign
///     options:
///       dependsOn:
///         - ${exampleWorkspace}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Example HCL Configurations
///
/// * Databricks Workspace with Root Databricks File System Customer Managed Keys
/// * Databricks Workspace with Root Databricks File System Customer Managed Keys in a Different Subscription
/// * Databricks Workspace with Private Endpoint, Customer Managed Keys for Managed Services and Root Databricks File System Customer Managed Keys
///
/// ## Import
///
/// Databricks Workspace Root DBFS Customer Managed Key can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:databricks/workspaceRootDbfsCustomerManagedKey:WorkspaceRootDbfsCustomerManagedKey workspace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Databricks/workspaces/workspace1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod workspace_root_dbfs_customer_managed_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceRootDbfsCustomerManagedKeyArgs {
        #[builder(into, default)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Key Vault Key to be used.
        #[builder(into)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Databricks Workspace.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceRootDbfsCustomerManagedKeyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the Key Vault Key to be used.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Databricks Workspace.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceRootDbfsCustomerManagedKeyArgs,
    ) -> WorkspaceRootDbfsCustomerManagedKeyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceRootDbfsCustomerManagedKeyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> WorkspaceRootDbfsCustomerManagedKeyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceRootDbfsCustomerManagedKeyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> WorkspaceRootDbfsCustomerManagedKeyResult {
        let key_vault_id_binding = args.key_vault_id.get_output(ctx);
        let key_vault_key_id_binding = args.key_vault_key_id.get_output(ctx);
        let workspace_id_binding = args.workspace_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:databricks/workspaceRootDbfsCustomerManagedKey:WorkspaceRootDbfsCustomerManagedKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        WorkspaceRootDbfsCustomerManagedKeyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            key_vault_id: o.get_field("keyVaultId"),
            key_vault_key_id: o.get_field("keyVaultKeyId"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
