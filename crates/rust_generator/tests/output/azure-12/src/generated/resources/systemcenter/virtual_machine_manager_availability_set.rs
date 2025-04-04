/// Manages a System Center Virtual Machine Manager Availability Set.
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
///     let exampleVirtualMachineManagerAvailabilitySet = virtual_machine_manager_availability_set::create(
///         "exampleVirtualMachineManagerAvailabilitySet",
///         VirtualMachineManagerAvailabilitySetArgs::builder()
///             .custom_location_id("${exampleVirtualMachineManagerServer.customLocationId}")
///             .location("${example.location}")
///             .name("example-scvmmas")
///             .resource_group_name("${example.name}")
///             .system_center_virtual_machine_manager_server_id(
///                 "${exampleVirtualMachineManagerServer.id}",
///             )
///             .build_struct(),
///     );
///     let exampleVirtualMachineManagerServer = virtual_machine_manager_server::create(
///         "exampleVirtualMachineManagerServer",
///         VirtualMachineManagerServerArgs::builder()
///             .custom_location_id(
///                 "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.ExtendedLocation/customLocations/customLocation1",
///             )
///             .fqdn("example.labtest")
///             .location("${example.location}")
///             .name("example-scvmmms")
///             .password("H@Sh1CoR3!")
///             .resource_group_name("${example.name}")
///             .username("testUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// System Center Virtual Machine Manager Availability Sets can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:systemcenter/virtualMachineManagerAvailabilitySet:VirtualMachineManagerAvailabilitySet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ScVmm/availabilitySets/availabilitySet1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine_manager_availability_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineManagerAvailabilitySetArgs {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Availability Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the System Center Virtual Machine Availability Set should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub system_center_virtual_machine_manager_server_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Availability Set.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineManagerAvailabilitySetResult {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Availability Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the System Center Virtual Machine Availability Set should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub system_center_virtual_machine_manager_server_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Availability Set.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineManagerAvailabilitySetArgs,
    ) -> VirtualMachineManagerAvailabilitySetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_location_id_binding = args.custom_location_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let system_center_virtual_machine_manager_server_id_binding = args
            .system_center_virtual_machine_manager_server_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:systemcenter/virtualMachineManagerAvailabilitySet:VirtualMachineManagerAvailabilitySet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding.drop_type(),
                },
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
                    name: "systemCenterVirtualMachineManagerServerId".into(),
                    value: &system_center_virtual_machine_manager_server_id_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineManagerAvailabilitySetResult {
            custom_location_id: o.get_field("customLocationId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            system_center_virtual_machine_manager_server_id: o
                .get_field("systemCenterVirtualMachineManagerServerId"),
            tags: o.get_field("tags"),
        }
    }
}
