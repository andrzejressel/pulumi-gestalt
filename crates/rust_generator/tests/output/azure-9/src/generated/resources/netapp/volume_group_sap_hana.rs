/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: random:RandomString
///     properties:
///       length: 12
///       special: true
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: ${prefix}-resources
///       location: ${location}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: ${prefix}-vnet
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       addressSpaces:
///         - 10.6.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: ${prefix}-delegated-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.6.2.0/24
///       delegations:
///         - name: testdelegation
///           serviceDelegation:
///             name: Microsoft.Netapp/volumes
///             actions:
///               - Microsoft.Network/networkinterfaces/*
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   example1:
///     type: azure:network:Subnet
///     properties:
///       name: ${prefix}-hosts-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.6.1.0/24
///   examplePlacementGroup:
///     type: azure:proximity:PlacementGroup
///     name: example
///     properties:
///       name: ${prefix}-ppg
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleAvailabilitySet:
///     type: azure:compute:AvailabilitySet
///     name: example
///     properties:
///       name: ${prefix}-avset
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       proximityPlacementGroupId: ${examplePlacementGroup.id}
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: ${prefix}-nic
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${example1.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: ${prefix}-vm
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       size: Standard_M8ms
///       adminUsername: ${adminUsername}
///       adminPassword: ${adminPassword}
///       disablePasswordAuthentication: false
///       proximityPlacementGroupId: ${examplePlacementGroup.id}
///       availabilitySetId: ${exampleAvailabilitySet.id}
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       osDisk:
///         storageAccountType: Standard_LRS
///         caching: ReadWrite
///   exampleAccount:
///     type: azure:netapp:Account
///     name: example
///     properties:
///       name: ${prefix}-netapp-account
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///     options:
///       dependsOn:
///         - ${exampleSubnet}
///         - ${example1}
///   examplePool:
///     type: azure:netapp:Pool
///     name: example
///     properties:
///       name: ${prefix}-netapp-pool
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       accountName: ${exampleAccount.name}
///       serviceLevel: Standard
///       sizeInTb: 8
///       qosType: Manual
///   exampleVolumeGroupSapHana:
///     type: azure:netapp:VolumeGroupSapHana
///     name: example
///     properties:
///       name: ${prefix}-netapp-volumegroup
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       accountName: ${exampleAccount.name}
///       groupDescription: Test volume group
///       applicationIdentifier: TST
///       volumes:
///         - name: ${prefix}-netapp-volume-1
///           volumePath: my-unique-file-path-1
///           serviceLevel: Standard
///           capacityPoolId: ${examplePool.id}
///           subnetId: ${exampleSubnet.id}
///           proximityPlacementGroupId: ${examplePlacementGroup.id}
///           volumeSpecName: data
///           storageQuotaInGb: 1024
///           throughputInMibps: 24
///           protocols: NFSv4.1
///           securityStyle: unix
///           snapshotDirectoryVisible: false
///           exportPolicyRules:
///             - ruleIndex: 1
///               allowedClients: 0.0.0.0/0
///               nfsv3Enabled: false
///               nfsv41Enabled: true
///               unixReadOnly: false
///               unixReadWrite: true
///               rootAccessEnabled: false
///           tags:
///             foo: bar
///         - name: ${prefix}-netapp-volume-2
///           volumePath: my-unique-file-path-2
///           serviceLevel: Standard
///           capacityPoolId: ${examplePool.id}
///           subnetId: ${exampleSubnet.id}
///           proximityPlacementGroupId: ${examplePlacementGroup.id}
///           volumeSpecName: log
///           storageQuotaInGb: 1024
///           throughputInMibps: 24
///           protocols: NFSv4.1
///           securityStyle: unix
///           snapshotDirectoryVisible: false
///           exportPolicyRules:
///             - ruleIndex: 1
///               allowedClients: 0.0.0.0/0
///               nfsv3Enabled: false
///               nfsv41Enabled: true
///               unixReadOnly: false
///               unixReadWrite: true
///               rootAccessEnabled: false
///           tags:
///             foo: bar
///         - name: ${prefix}-netapp-volume-3
///           volumePath: my-unique-file-path-3
///           serviceLevel: Standard
///           capacityPoolId: ${examplePool.id}
///           subnetId: ${exampleSubnet.id}
///           proximityPlacementGroupId: ${examplePlacementGroup.id}
///           volumeSpecName: shared
///           storageQuotaInGb: 1024
///           throughputInMibps: 24
///           protocols: NFSv4.1
///           securityStyle: unix
///           snapshotDirectoryVisible: false
///           exportPolicyRules:
///             - ruleIndex: 1
///               allowedClients: 0.0.0.0/0
///               nfsv3Enabled: false
///               nfsv41Enabled: true
///               unixReadOnly: false
///               unixReadWrite: true
///               rootAccessEnabled: false
///     options:
///       dependsOn:
///         - ${exampleLinuxVirtualMachine}
///         - ${examplePlacementGroup}
/// variables:
///   adminUsername: exampleadmin
///   adminPassword: ${example.result}
/// ```
///
/// ## Import
///
/// Application Volume Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/volumeGroupSapHana:VolumeGroupSapHana example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mytest-rg/providers/Microsoft.NetApp/netAppAccounts/netapp-account-test/volumeGroups/netapp-volumegroup-test
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod volume_group_sap_hana {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeGroupSapHanaArgs {
        /// Name of the account where the application volume group belong to. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SAP System ID, maximum 3 characters, e.g. `SH9`. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub application_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Volume group description. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub group_description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Application Volume Group. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `volume` blocks as defined below.
        #[builder(into)]
        pub volumes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::netapp::VolumeGroupSapHanaVolume>,
        >,
    }
    #[allow(dead_code)]
    pub struct VolumeGroupSapHanaResult {
        /// Name of the account where the application volume group belong to. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The SAP System ID, maximum 3 characters, e.g. `SH9`. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub application_identifier: pulumi_gestalt_rust::Output<String>,
        /// Volume group description. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub group_description: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Application Volume Group. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `volume` blocks as defined below.
        pub volumes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::netapp::VolumeGroupSapHanaVolume>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VolumeGroupSapHanaArgs,
    ) -> VolumeGroupSapHanaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let application_identifier_binding = args
            .application_identifier
            .get_output(context);
        let group_description_binding = args.group_description.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let volumes_binding = args.volumes.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:netapp/volumeGroupSapHana:VolumeGroupSapHana".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationIdentifier".into(),
                    value: &application_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupDescription".into(),
                    value: &group_description_binding.drop_type(),
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
                    name: "volumes".into(),
                    value: &volumes_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VolumeGroupSapHanaResult {
            account_name: o.get_field("accountName"),
            application_identifier: o.get_field("applicationIdentifier"),
            group_description: o.get_field("groupDescription"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            volumes: o.get_field("volumes"),
        }
    }
}
