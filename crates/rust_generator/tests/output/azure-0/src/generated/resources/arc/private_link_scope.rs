/// Manages an Azure Arc Private Link Scope.
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
///             .location("west europe")
///             .name("rg-example")
///             .build_struct(),
///     );
///     let examplePrivateLinkScope = private_link_scope::create(
///         "examplePrivateLinkScope",
///         PrivateLinkScopeArgs::builder()
///             .location("${example.location}")
///             .name("plsexample")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Arc Private Link Scope can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arc/privateLinkScope:PrivateLinkScope example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.HybridCompute/privateLinkScopes/privateLinkScope1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod private_link_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkScopeArgs {
        /// The Azure Region where the Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name which should be used for the Azure Arc Private Link Scope. Changing this forces a new Azure Arc Private Link Scope to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Indicates whether machines associated with the private link scope can also use public Azure Arc service endpoints. Defaults to `false`. Possible values are `true` and `false`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The name of the Resource Group where the Azure Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// A mapping of tags which should be assigned to the Azure Arc Private Link Scope.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkScopeResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for the Azure Arc Private Link Scope. Changing this forces a new Azure Arc Private Link Scope to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether machines associated with the private link scope can also use public Azure Arc service endpoints. Defaults to `false`. Possible values are `true` and `false`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Azure Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Arc Private Link Scope.
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
        args: PrivateLinkScopeArgs,
    ) -> PrivateLinkScopeResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkScopeArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> PrivateLinkScopeResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkScopeArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> PrivateLinkScopeResult {
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:arc/privateLinkScope:PrivateLinkScope".into(),
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        PrivateLinkScopeResult {
            id: o.get_id(),
            urn: o.get_urn(),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
