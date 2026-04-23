/// Manages a custom Role Definition, used to assign Roles to Users/Principals. See ['Understand role definitions'](https://docs.microsoft.com/azure/role-based-access-control/role-definitions) in the Azure documentation for more details.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:authorization:RoleDefinition
///     properties:
///       name: my-custom-role
///       scope: ${primary.id}
///       description: This is a custom role created
///       permissions:
///         - actions:
///             - '*'
///           notActions: []
///       assignableScopes:
///         - ${primary.id}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
///
/// ### With Management Group
/// ```yaml
/// resources:
///   example:
///     type: azure:management:Group
///     properties:
///       displayName: ParentGroup
///       subscriptionIds:
///         - ${current.subscriptionId}
///   exampleRoleDefinition:
///     type: azure:authorization:RoleDefinition
///     name: example
///     properties:
///       name: example-mg-role
///       scope: ${example.id}
///       description: Example custom role scoped to a management group.
///       permissions:
///         - actions:
///             - Microsoft.Insights/alertRules/*
///           notActions: []
///       assignableScopes:
///         - ${example.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Role Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:role/definition:Definition example "/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleDefinitions/00000000-0000-0000-0000-000000000000|/subscriptions/00000000-0000-0000-0000-000000000000"
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// One or more assignable scopes for this Role Definition, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333` , or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`.
        ///
        /// > **NOTE:** The value for `scope` is automatically included in this list if no other values supplied.
        #[builder(into, default)]
        pub assignable_scopes: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// A description of the Role Definition.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Role Definition.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::Input<
            Option<Vec<super::super::types::role::DefinitionPermission>>,
        >,
        /// A unique UUID/GUID which identifies this role - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub role_definition_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// The scope at which the Role Definition applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`. It is recommended to use the first entry of the `assignable_scopes`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// One or more assignable scopes for this Role Definition, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333` , or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`.
        ///
        /// > **NOTE:** The value for `scope` is automatically included in this list if no other values supplied.
        pub assignable_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A description of the Role Definition.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Role Definition.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `permissions` block as defined below.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::role::DefinitionPermission>>,
        >,
        /// A unique UUID/GUID which identifies this role - one will be generated if not specified. Changing this forces a new resource to be created.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Resource Manager ID for the resource.
        pub role_definition_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The scope at which the Role Definition applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`. It is recommended to use the first entry of the `assignable_scopes`. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefinitionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DefinitionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefinitionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DefinitionResult {
        let assignable_scopes_binding = args.assignable_scopes.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let permissions_binding = args.permissions.get_output(ctx);
        let role_definition_id_binding = args.role_definition_id.get_output(ctx);
        let scope_binding = args.scope.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:role/definition:Definition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignableScopes".into(),
                    value: &assignable_scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DefinitionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            assignable_scopes: o.get_field("assignableScopes"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            role_definition_id: o.get_field("roleDefinitionId"),
            role_definition_resource_id: o.get_field("roleDefinitionResourceId"),
            scope: o.get_field("scope"),
        }
    }
}
