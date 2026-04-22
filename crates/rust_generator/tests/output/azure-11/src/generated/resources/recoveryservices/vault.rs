/// Manages a Recovery Services Vault.
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
///     let vault = vault::create(
///         "vault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("example-recovery-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .soft_delete_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Recovery Services Vaults can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:recoveryservices/vault:Vault vault1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/vault1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultArgs {
        /// Whether to enable the Classic experience for VMware replication. If set to `false` VMware machines will be protected using the new stateless ASR replication appliance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub classic_vmware_replication_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Is cross region restore enabled for this Vault? Only can be `true`, when `storage_mode_type` is `GeoRedundant`. Defaults to `false`.
        ///
        /// > **Note:** Once `cross_region_restore_enabled` is set to `true`, changing it back to `false` forces a new Recovery Service Vault to be created.
        #[builder(into, default)]
        pub cross_region_restore_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// An `encryption` block as defined below. Required with `identity`.
        ///
        /// !> **Note:** Once Encryption with your own key has been Enabled it's not possible to Disable it.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::Input<
            Option<super::super::types::recoveryservices::VaultEncryption>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::Input<
            Option<super::super::types::recoveryservices::VaultIdentity>,
        >,
        /// Immutability Settings of vault, possible values include: `Locked`, `Unlocked` and `Disabled`.
        ///
        /// > **Note:** Once `immutability` is set to `Locked`, changing it to other values forces a new Recovery Services Vault to be created.
        #[builder(into, default)]
        pub immutability: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// A `monitoring` block as defined below.
        #[builder(into, default)]
        pub monitoring: pulumi_gestalt_rust::Input<
            Option<super::super::types::recoveryservices::VaultMonitoring>,
        >,
        /// Specifies the name of the Recovery Services Vault. Recovery Service Vault name must be 2 - 50 characters long, start with a letter, contain only letters, numbers and hyphens. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Is it enabled to access the vault from public networks. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The name of the resource group in which to create the Recovery Services Vault. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Sets the vault's SKU. Possible values include: `Standard`, `RS0`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::Input<String>,
        /// Is soft delete enable for this Vault? Defaults to `true`.
        #[builder(into, default)]
        pub soft_delete_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The storage type of the Recovery Services Vault. Possible values are `GeoRedundant`, `LocallyRedundant` and `ZoneRedundant`. Defaults to `GeoRedundant`.
        #[builder(into, default)]
        pub storage_mode_type: pulumi_gestalt_rust::Input<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VaultResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable the Classic experience for VMware replication. If set to `false` VMware machines will be protected using the new stateless ASR replication appliance. Changing this forces a new resource to be created.
        pub classic_vmware_replication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Is cross region restore enabled for this Vault? Only can be `true`, when `storage_mode_type` is `GeoRedundant`. Defaults to `false`.
        ///
        /// > **Note:** Once `cross_region_restore_enabled` is set to `true`, changing it back to `false` forces a new Recovery Service Vault to be created.
        pub cross_region_restore_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `encryption` block as defined below. Required with `identity`.
        ///
        /// !> **Note:** Once Encryption with your own key has been Enabled it's not possible to Disable it.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::recoveryservices::VaultEncryption>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::recoveryservices::VaultIdentity>,
        >,
        /// Immutability Settings of vault, possible values include: `Locked`, `Unlocked` and `Disabled`.
        ///
        /// > **Note:** Once `immutability` is set to `Locked`, changing it to other values forces a new Recovery Services Vault to be created.
        pub immutability: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `monitoring` block as defined below.
        pub monitoring: pulumi_gestalt_rust::Output<
            Option<super::super::types::recoveryservices::VaultMonitoring>,
        >,
        /// Specifies the name of the Recovery Services Vault. Recovery Service Vault name must be 2 - 50 characters long, start with a letter, contain only letters, numbers and hyphens. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is it enabled to access the vault from public networks. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Recovery Services Vault. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Sets the vault's SKU. Possible values include: `Standard`, `RS0`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// Is soft delete enable for this Vault? Defaults to `true`.
        pub soft_delete_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The storage type of the Recovery Services Vault. Possible values are `GeoRedundant`, `LocallyRedundant` and `ZoneRedundant`. Defaults to `GeoRedundant`.
        pub storage_mode_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultArgs,
    ) -> VaultResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VaultResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VaultResult {
        let classic_vmware_replication_enabled_binding = args
            .classic_vmware_replication_enabled
            .get_output(ctx);
        let cross_region_restore_enabled_binding = args
            .cross_region_restore_enabled
            .get_output(ctx);
        let encryption_binding = args.encryption.get_output(ctx);
        let identity_binding = args.identity.get_output(ctx);
        let immutability_binding = args.immutability.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let monitoring_binding = args.monitoring.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let sku_binding = args.sku.get_output(ctx);
        let soft_delete_enabled_binding = args.soft_delete_enabled.get_output(ctx);
        let storage_mode_type_binding = args.storage_mode_type.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:recoveryservices/vault:Vault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "classicVmwareReplicationEnabled".into(),
                    value: &classic_vmware_replication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "crossRegionRestoreEnabled".into(),
                    value: &cross_region_restore_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "immutability".into(),
                    value: &immutability_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoring".into(),
                    value: &monitoring_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "softDeleteEnabled".into(),
                    value: &soft_delete_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageModeType".into(),
                    value: &storage_mode_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VaultResult {
            id: o.get_id(),
            urn: o.get_urn(),
            classic_vmware_replication_enabled: o
                .get_field("classicVmwareReplicationEnabled"),
            cross_region_restore_enabled: o.get_field("crossRegionRestoreEnabled"),
            encryption: o.get_field("encryption"),
            identity: o.get_field("identity"),
            immutability: o.get_field("immutability"),
            location: o.get_field("location"),
            monitoring: o.get_field("monitoring"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            soft_delete_enabled: o.get_field("softDeleteEnabled"),
            storage_mode_type: o.get_field("storageModeType"),
            tags: o.get_field("tags"),
        }
    }
}
