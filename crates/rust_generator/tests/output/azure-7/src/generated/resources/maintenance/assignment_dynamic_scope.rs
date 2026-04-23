/// Manages a Dynamic Maintenance Assignment.
///
/// > **Note:** Only valid for `InGuestPatch` Maintenance Configuration Scopes.
///
/// ## Import
///
/// Dynamic Maintenance Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maintenance/assignmentDynamicScope:AssignmentDynamicScope example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Maintenance/configurationAssignments/assignmentName
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod assignment_dynamic_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentDynamicScopeArgs {
        /// A `filter` block as defined below.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::Input<
            super::super::types::maintenance::AssignmentDynamicScopeFilter,
        >,
        /// The ID of the Maintenance Configuration Resource. Changing this forces a new Dynamic Maintenance Assignment to be created.
        #[builder(into)]
        pub maintenance_configuration_id: pulumi_gestalt_rust::Input<String>,
        /// The name which should be used for this Dynamic Maintenance Assignment. Changing this forces a new Dynamic Maintenance Assignment to be created.
        ///
        /// > **Note:** The `name` must be unique per subscription.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssignmentDynamicScopeResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A `filter` block as defined below.
        pub filter: pulumi_gestalt_rust::Output<
            super::super::types::maintenance::AssignmentDynamicScopeFilter,
        >,
        /// The ID of the Maintenance Configuration Resource. Changing this forces a new Dynamic Maintenance Assignment to be created.
        pub maintenance_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Dynamic Maintenance Assignment. Changing this forces a new Dynamic Maintenance Assignment to be created.
        ///
        /// > **Note:** The `name` must be unique per subscription.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentDynamicScopeArgs,
    ) -> AssignmentDynamicScopeResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentDynamicScopeArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AssignmentDynamicScopeResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentDynamicScopeArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AssignmentDynamicScopeResult {
        let filter_binding = args.filter.get_output(ctx);
        let maintenance_configuration_id_binding = args
            .maintenance_configuration_id
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:maintenance/assignmentDynamicScope:AssignmentDynamicScope"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfigurationId".into(),
                    value: &maintenance_configuration_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AssignmentDynamicScopeResult {
            id: o.get_id(),
            urn: o.get_urn(),
            filter: o.get_field("filter"),
            maintenance_configuration_id: o.get_field("maintenanceConfigurationId"),
            name: o.get_field("name"),
        }
    }
}
