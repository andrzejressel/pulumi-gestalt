/// Manages a Trusted Signing Account.
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
///             .name("example")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("West Europe")
///             .name("example-account")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Trusted Signing Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:trustedsigning/account:Account example /subscriptions/0000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.CodeSigning/codeSigningAccounts/example-account
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// The Azure Region where the Trusted Signing Account should exist. Changing this forces a new Trusted Signing Account to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name which should be used for this Trusted Signing Account. Changing this forces a new Trusted Signing Account to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group where the Trusted Signing Account should exist. Changing this forces a new Trusted Signing Account to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The sku name of this Trusted Signing Account. Possible values are `Basic` and `Premium`.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::Input<String>,
        /// A mapping of tags which should be assigned to the Trusted Signing Account.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The URI of the Trusted Signing Account which is used during signing files.
        pub account_uri: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Trusted Signing Account should exist. Changing this forces a new Trusted Signing Account to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Trusted Signing Account. Changing this forces a new Trusted Signing Account to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Trusted Signing Account should exist. Changing this forces a new Trusted Signing Account to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The sku name of this Trusted Signing Account. Possible values are `Basic` and `Premium`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Trusted Signing Account.
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
        args: AccountArgs,
    ) -> AccountResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AccountResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AccountResult {
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let sku_name_binding = args.sku_name.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:trustedsigning/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AccountResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_uri: o.get_field("accountUri"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
        }
    }
}
