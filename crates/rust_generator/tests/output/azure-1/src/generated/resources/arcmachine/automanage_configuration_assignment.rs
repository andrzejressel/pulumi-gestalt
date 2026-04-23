/// Manages an Arc Machine Automanage Configuration Profile Assignment.
///
/// ## Example Usage
///
/// ```yaml
/// configuration:
///   arcMachineName:
///     type: dynamic
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleConfiguration:
///     type: azure:automanage:Configuration
///     name: example
///     properties:
///       name: example-configuration
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleAutomanageConfigurationAssignment:
///     type: azure:arcmachine:AutomanageConfigurationAssignment
///     name: example
///     properties:
///       arcMachineId: ${example.id}
///       configurationId: ${exampleConfiguration.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:arcmachine:get
///       arguments:
///         name: ${arcMachineName}
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Virtual Machine Automanage Configuration Profile Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arcmachine/automanageConfigurationAssignment:AutomanageConfigurationAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.HybridCompute/machines/machine1/providers/Microsoft.AutoManage/configurationProfileAssignments/default
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod automanage_configuration_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentArgs {
        /// The ARM resource ID of the Arc Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_machine_id: pulumi_gestalt_rust::Input<String>,
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** For a successful creation of this resource, locate "Automanage API Access" app within your Entra ID tenant. Make sure it's granted access to the scope that includes the arc server.
        #[builder(into)]
        pub configuration_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARM resource ID of the Arc Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        pub arc_machine_id: pulumi_gestalt_rust::Output<String>,
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** For a successful creation of this resource, locate "Automanage API Access" app within your Entra ID tenant. Make sure it's granted access to the scope that includes the arc server.
        pub configuration_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutomanageConfigurationAssignmentArgs,
    ) -> AutomanageConfigurationAssignmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutomanageConfigurationAssignmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AutomanageConfigurationAssignmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutomanageConfigurationAssignmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AutomanageConfigurationAssignmentResult {
        let arc_machine_id_binding = args.arc_machine_id.get_output(ctx);
        let configuration_id_binding = args.configuration_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:arcmachine/automanageConfigurationAssignment:AutomanageConfigurationAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arcMachineId".into(),
                    value: &arc_machine_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationId".into(),
                    value: &configuration_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AutomanageConfigurationAssignmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arc_machine_id: o.get_field("arcMachineId"),
            configuration_id: o.get_field("configurationId"),
        }
    }
}
