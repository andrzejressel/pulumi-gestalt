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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod automanage_configuration_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentArgs {
        /// The ARM resource ID of the Arc Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** For a successful creation of this resource, locate "Automanage API Access" app within your Entra ID tenant. Make sure it's granted access to the scope that includes the arc server.
        #[builder(into)]
        pub configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutomanageConfigurationAssignmentArgs,
    ) -> AutomanageConfigurationAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arc_machine_id_binding = args.arc_machine_id.get_output(context);
        let configuration_id_binding = args.configuration_id.get_output(context);
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
        };
        let o = context.register_resource(request);
        AutomanageConfigurationAssignmentResult {
            id: o.get_field("id"),
            arc_machine_id: o.get_field("arcMachineId"),
            configuration_id: o.get_field("configurationId"),
        }
    }
}
