/// Manages a maintenance assignment to a virtual machine scale set.
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
///     let exampleAssignmentVirtualMachineScaleSet = assignment_virtual_machine_scale_set::create(
///         "exampleAssignmentVirtualMachineScaleSet",
///         AssignmentVirtualMachineScaleSetArgs::builder()
///             .location("${example.location}")
///             .maintenance_configuration_id("${exampleConfiguration.id}")
///             .virtual_machine_scale_set_id("${exampleLinuxVirtualMachine.id}")
///             .build_struct(),
///     );
///     let exampleBackendAddressPool = backend_address_pool::create(
///         "exampleBackendAddressPool",
///         BackendAddressPoolArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleConfiguration = configuration::create(
///         "exampleConfiguration",
///         ConfigurationArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .scope("OSImage")
///             .visibility("Custom")
///             .window(
///                 ConfigurationWindow::builder()
///                     .duration("06:00")
///                     .expirationDateTime("9999-12-31 00:00")
///                     .recurEvery("1Days")
///                     .startDateTime("2021-12-31 00:00")
///                     .timeZone("Pacific Standard Time")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleLinuxVirtualMachine = linux_virtual_machine::create(
///         "exampleLinuxVirtualMachine",
///         LinuxVirtualMachineArgs::builder()
///             .admin_username("adminuser")
///             .location("${example.location}")
///             .name("example-machine")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_disk(
///                 LinuxVirtualMachineOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .size("Standard_F2")
///             .build_struct(),
///     );
///     let exampleLinuxVirtualMachineScaleSet = linux_virtual_machine_scale_set::create(
///         "exampleLinuxVirtualMachineScaleSet",
///         LinuxVirtualMachineScaleSetArgs::builder()
///             .admin_password("P@ssword1234!")
///             .admin_username("adminuser")
///             .automatic_os_upgrade_policy(
///                 LinuxVirtualMachineScaleSetAutomaticOsUpgradePolicy::builder()
///                     .disableAutomaticRollback(true)
///                     .enableAutomaticOsUpgrade(true)
///                     .build_struct(),
///             )
///             .disable_password_authentication(false)
///             .health_probe_id("${exampleProbe.id}")
///             .instances(1)
///             .location("${example.location}")
///             .name("example")
///             .network_interfaces(
///                 vec![
///                     LinuxVirtualMachineScaleSetNetworkInterface::builder()
///                     .ipConfigurations(vec![LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration::builder()
///                     .loadBalancerBackendAddressPoolIds(vec!["${exampleBackendAddressPool.id}",])
///                     .name("internal").primary(true).subnetId("${exampleSubnet.id}")
///                     .build_struct(),]).name("example").primary(true).build_struct(),
///                 ],
///             )
///             .os_disk(
///                 LinuxVirtualMachineScaleSetOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .rolling_upgrade_policy(
///                 LinuxVirtualMachineScaleSetRollingUpgradePolicy::builder()
///                     .maxBatchInstancePercent(20)
///                     .maxUnhealthyInstancePercent(20)
///                     .maxUnhealthyUpgradedInstancePercent(20)
///                     .pauseTimeBetweenBatches("PT0S")
///                     .build_struct(),
///             )
///             .sku("Standard_F2")
///             .source_image_reference(
///                 LinuxVirtualMachineScaleSetSourceImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .upgrade_mode("Automatic")
///             .build_struct(),
///     );
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder().name("internal")
///                     .publicIpAddressId("${examplePublicIp.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("${example.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("testconfiguration1")
///                     .privateIpAddressAllocation("Dynamic").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("sample-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleProbe = probe::create(
///         "exampleProbe",
///         ProbeArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("example")
///             .port(22)
///             .protocol("Tcp")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("${example.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleRule = rule::create(
///         "exampleRule",
///         RuleArgs::builder()
///             .backend_port(22)
///             .frontend_ip_configuration_name("internal")
///             .frontend_port(22)
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("example")
///             .probe_id("${exampleProbe.id}")
///             .protocol("Tcp")
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
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Maintenance Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maintenance/assignmentVirtualMachineScaleSet:AssignmentVirtualMachineScaleSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Compute/virtualMachineScaleSets/vmss1/providers/Microsoft.Maintenance/configurationAssignments/assign1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assignment_virtual_machine_scale_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentVirtualMachineScaleSetArgs {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Maintenance Configuration Resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub maintenance_configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Virtual Machine Scale Set ID to which the Maintenance Configuration will be assigned. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_scale_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssignmentVirtualMachineScaleSetResult {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Maintenance Configuration Resource. Changing this forces a new resource to be created.
        pub maintenance_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Virtual Machine Scale Set ID to which the Maintenance Configuration will be assigned. Changing this forces a new resource to be created.
        pub virtual_machine_scale_set_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentVirtualMachineScaleSetArgs,
    ) -> AssignmentVirtualMachineScaleSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let maintenance_configuration_id_binding = args
            .maintenance_configuration_id
            .get_output(context);
        let virtual_machine_scale_set_id_binding = args
            .virtual_machine_scale_set_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:maintenance/assignmentVirtualMachineScaleSet:AssignmentVirtualMachineScaleSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfigurationId".into(),
                    value: &maintenance_configuration_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineScaleSetId".into(),
                    value: &virtual_machine_scale_set_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssignmentVirtualMachineScaleSetResult {
            location: o.get_field("location"),
            maintenance_configuration_id: o.get_field("maintenanceConfigurationId"),
            virtual_machine_scale_set_id: o.get_field("virtualMachineScaleSetId"),
        }
    }
}
