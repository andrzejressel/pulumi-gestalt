/// Manages an Azure Monitor Private Link Scoped Service.
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
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("example-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePrivateLinkScope = private_link_scope::create(
///         "examplePrivateLinkScope",
///         PrivateLinkScopeArgs::builder()
///             .name("example-ampls")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePrivateLinkScopedService = private_link_scoped_service::create(
///         "examplePrivateLinkScopedService",
///         PrivateLinkScopedServiceArgs::builder()
///             .linked_resource_id("${exampleInsights.id}")
///             .name("example-amplsservice")
///             .resource_group_name("${example.name}")
///             .scope_name("${examplePrivateLinkScope.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Monitor Private Link Scoped Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/privateLinkScopedService:PrivateLinkScopedService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/privateLinkScopes/pls1/scopedResources/sr1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod private_link_scoped_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkScopedServiceArgs {
        /// The ID of the linked resource. It must be the Log Analytics workspace or the Application Insights component or the Data Collection endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub linked_resource_id: pulumi_gestalt_rust::Input<String>,
        /// The name of the Azure Monitor Private Link Scoped Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group where the Azure Monitor Private Link Scoped Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the Azure Monitor Private Link Scope. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkScopedServiceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the linked resource. It must be the Log Analytics workspace or the Application Insights component or the Data Collection endpoint. Changing this forces a new resource to be created.
        pub linked_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Monitor Private Link Scoped Service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Monitor Private Link Scoped Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Monitor Private Link Scope. Changing this forces a new resource to be created.
        pub scope_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkScopedServiceArgs,
    ) -> PrivateLinkScopedServiceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkScopedServiceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> PrivateLinkScopedServiceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkScopedServiceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> PrivateLinkScopedServiceResult {
        let linked_resource_id_binding = args.linked_resource_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let scope_name_binding = args.scope_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/privateLinkScopedService:PrivateLinkScopedService"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedResourceId".into(),
                    value: &linked_resource_id_binding.drop_type(),
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
                    name: "scopeName".into(),
                    value: &scope_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        PrivateLinkScopedServiceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            linked_resource_id: o.get_field("linkedResourceId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope_name: o.get_field("scopeName"),
        }
    }
}
