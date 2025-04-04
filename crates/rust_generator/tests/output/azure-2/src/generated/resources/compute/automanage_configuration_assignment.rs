/// Manages a Virtual Machine Automanage Configuration Profile Assignment.
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
///         ResourceGroupArgs::builder().location("westus").name("example-rg").build_struct(),
///     );
///     let exampleAutomanageConfigurationAssignment = automanage_configuration_assignment::create(
///         "exampleAutomanageConfigurationAssignment",
///         AutomanageConfigurationAssignmentArgs::builder()
///             .configuration_id("${exampleConfiguration.id}")
///             .virtual_machine_id("${exampleLinuxVirtualMachine.id}")
///             .build_struct(),
///     );
///     let exampleConfiguration = configuration::create(
///         "exampleConfiguration",
///         ConfigurationArgs::builder()
///             .location("${example.location}")
///             .name("exampleconfig")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinuxVirtualMachine = linux_virtual_machine::create(
///         "exampleLinuxVirtualMachine",
///         LinuxVirtualMachineArgs::builder()
///             .admin_password("P@$$w0rd1234!")
///             .admin_username("adminuser")
///             .disable_password_authentication(false)
///             .location("${example.location}")
///             .name("examplevm")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_disk(
///                 LinuxVirtualMachineOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .size("Standard_F2")
///             .source_image_reference(
///                 LinuxVirtualMachineSourceImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("internal")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("exampleni")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("examplevnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Machine Automanage Configuration Profile Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/automanageConfigurationAssignment:AutomanageConfigurationAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/virtualMachines/vm1/providers/Microsoft.AutoManage/configurationProfileAssignments/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod automanage_configuration_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentArgs {
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARM resource ID of the Virtual Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentResult {
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        pub configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The ARM resource ID of the Virtual Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
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
        let configuration_id_binding = args.configuration_id.get_output(context);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/automanageConfigurationAssignment:AutomanageConfigurationAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationId".into(),
                    value: &configuration_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutomanageConfigurationAssignmentResult {
            configuration_id: o.get_field("configurationId"),
            virtual_machine_id: o.get_field("virtualMachineId"),
        }
    }
}
