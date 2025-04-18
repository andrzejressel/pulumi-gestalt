#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_role_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoleDefinitionArgs {
        /// Specifies the Name of either a built-in or custom Role Definition.
        ///
        /// > You can also use this for built-in roles such as `Contributor`, `Owner`, `Reader` and `Virtual Machine Contributor`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Role Definition as a UUID/GUID.
        #[builder(into, default)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Scope at which the Custom Role Definition exists.
        ///
        /// > **Note:** One of `name` or `role_definition_id` must be specified.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRoleDefinitionResult {
        /// One or more assignable scopes for this Role Definition, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`.
        pub assignable_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Description of the built-in Role.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `permissions` block as documented below.
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::authorization::GetRoleDefinitionPermission>,
        >,
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Type of the Role.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRoleDefinitionArgs,
    ) -> GetRoleDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:authorization/getRoleDefinition:getRoleDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
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
        };
        let o = context.invoke_resource(request);
        GetRoleDefinitionResult {
            assignable_scopes: o.get_field("assignableScopes"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            role_definition_id: o.get_field("roleDefinitionId"),
            scope: o.get_field("scope"),
            type_: o.get_field("type"),
        }
    }
}
