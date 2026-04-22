/// Manages a HyperV Site in Recovery Service Vault.
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
///     let exampleHyperVSite = hyper_v_site::create(
///         "exampleHyperVSite",
///         HyperVSiteArgs::builder()
///             .name("example-site")
///             .recovery_vault_id("${exampleVault.id}")
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder()
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
/// Recovery Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/hyperVSite:HyperVSite example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/vault1/replicationFabrics/fabric1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod hyper_v_site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HyperVSiteArgs {
        /// The name which should be used for this Recovery Service. Changing this forces a new Site to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the Recovery Services Vault where the Site created. Changing this forces a new Site to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct HyperVSiteResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Recovery Service. Changing this forces a new Site to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Recovery Services Vault where the Site created. Changing this forces a new Site to be created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HyperVSiteArgs,
    ) -> HyperVSiteResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HyperVSiteArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> HyperVSiteResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HyperVSiteArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> HyperVSiteResult {
        let name_binding = args.name.get_output(ctx);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/hyperVSite:HyperVSite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        HyperVSiteResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
        }
    }
}
