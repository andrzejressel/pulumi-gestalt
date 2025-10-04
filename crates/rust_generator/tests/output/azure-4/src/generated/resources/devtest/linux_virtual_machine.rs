/// Manages a Linux Virtual Machine within a Dev Test Lab.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: example-devtestlab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         Sydney: Australia
///   exampleVirtualNetwork:
///     type: azure:devtest:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       labName: ${exampleLab.name}
///       resourceGroupName: ${example.name}
///       subnet:
///         usePublicIpAddress: Allow
///         useInVirtualMachineCreation: Allow
///   exampleLinuxVirtualMachine:
///     type: azure:devtest:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example-vm03
///       labName: ${exampleLab.name}
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_DS2
///       username: exampleuser99
///       sshKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ~/.ssh/id_rsa.pub
///           return: result
///       labVirtualNetworkId: ${exampleVirtualNetwork.id}
///       labSubnetName: ${exampleVirtualNetwork.subnet.name}
///       storageType: Premium
///       notes: Some notes about this Virtual Machine.
///       galleryImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
/// ```
///
/// ## Import
///
/// Dev Test Linux Virtual Machines can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/linuxVirtualMachine:LinuxVirtualMachine machine1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevTestLab/labs/lab1/virtualMachines/machine1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linux_virtual_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinuxVirtualMachineArgs {
        /// Can this Virtual Machine be claimed by users? Defaults to `true`.
        #[builder(into, default)]
        pub allow_claim: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Virtual Machine be created without a Public IP Address? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub disallow_public_ip_address: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `gallery_image_reference` block as defined below.
        #[builder(into)]
        pub gallery_image_reference: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::devtest::LinuxVirtualMachineGalleryImageReference,
        >,
        /// One or more `inbound_nat_rule` blocks as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If any `inbound_nat_rule` blocks are specified then `disallow_public_ip_address` must be set to `true`.
        #[builder(into, default)]
        pub inbound_nat_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::devtest::LinuxVirtualMachineInboundNatRule>>,
        >,
        /// Specifies the name of the Dev Test Lab in which the Virtual Machine should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of a Subnet within the Dev Test Virtual Network where this machine should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_subnet_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Dev Test Virtual Network where this Virtual Machine should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the Dev Test Lab exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Dev Test Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The validation requirements for the Name change based on the `os_type` used in this Virtual Machine. For a Linux VM the name must be between 1-62 characters, and for a Windows VM the name must be between 1-15 characters. It must begin and end with a letter or number, and cannot be all numbers.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Any notes about the Virtual Machine.
        #[builder(into, default)]
        pub notes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Password associated with the `username` used to login to this Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Machine Size to use for this Virtual Machine, such as `Standard_F2`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SSH Key associated with the `username` used to login to this Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One or either `password` or `ssh_key` must be specified.
        #[builder(into, default)]
        pub ssh_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of Storage to use on this Virtual Machine. Possible values are `Standard` and `Premium`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Username associated with the local administrator on this Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinuxVirtualMachineResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Can this Virtual Machine be claimed by users? Defaults to `true`.
        pub allow_claim: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Virtual Machine be created without a Public IP Address? Changing this forces a new resource to be created.
        pub disallow_public_ip_address: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The FQDN of the Virtual Machine.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// A `gallery_image_reference` block as defined below.
        pub gallery_image_reference: pulumi_gestalt_rust::Output<
            super::super::types::devtest::LinuxVirtualMachineGalleryImageReference,
        >,
        /// One or more `inbound_nat_rule` blocks as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If any `inbound_nat_rule` blocks are specified then `disallow_public_ip_address` must be set to `true`.
        pub inbound_nat_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::devtest::LinuxVirtualMachineInboundNatRule>>,
        >,
        /// Specifies the name of the Dev Test Lab in which the Virtual Machine should be created. Changing this forces a new resource to be created.
        pub lab_name: pulumi_gestalt_rust::Output<String>,
        /// The name of a Subnet within the Dev Test Virtual Network where this machine should exist. Changing this forces a new resource to be created.
        pub lab_subnet_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Dev Test Virtual Network where this Virtual Machine should be created. Changing this forces a new resource to be created.
        pub lab_virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the Dev Test Lab exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Dev Test Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The validation requirements for the Name change based on the `os_type` used in this Virtual Machine. For a Linux VM the name must be between 1-62 characters, and for a Windows VM the name must be between 1-15 characters. It must begin and end with a letter or number, and cannot be all numbers.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Any notes about the Virtual Machine.
        pub notes: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Password associated with the `username` used to login to this Virtual Machine. Changing this forces a new resource to be created.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Machine Size to use for this Virtual Machine, such as `Standard_F2`. Changing this forces a new resource to be created.
        pub size: pulumi_gestalt_rust::Output<String>,
        /// The SSH Key associated with the `username` used to login to this Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One or either `password` or `ssh_key` must be specified.
        pub ssh_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of Storage to use on this Virtual Machine. Possible values are `Standard` and `Premium`. Changing this forces a new resource to be created.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique immutable identifier of the Virtual Machine.
        pub unique_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Username associated with the local administrator on this Virtual Machine. Changing this forces a new resource to be created.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinuxVirtualMachineArgs,
    ) -> LinuxVirtualMachineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_claim_binding = args.allow_claim.get_output(context);
        let disallow_public_ip_address_binding = args
            .disallow_public_ip_address
            .get_output(context);
        let gallery_image_reference_binding = args
            .gallery_image_reference
            .get_output(context);
        let inbound_nat_rules_binding = args.inbound_nat_rules.get_output(context);
        let lab_name_binding = args.lab_name.get_output(context);
        let lab_subnet_name_binding = args.lab_subnet_name.get_output(context);
        let lab_virtual_network_id_binding = args
            .lab_virtual_network_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let notes_binding = args.notes.get_output(context);
        let password_binding = args.password.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let size_binding = args.size.get_output(context);
        let ssh_key_binding = args.ssh_key.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devtest/linuxVirtualMachine:LinuxVirtualMachine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowClaim".into(),
                    value: &allow_claim_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disallowPublicIpAddress".into(),
                    value: &disallow_public_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryImageReference".into(),
                    value: &gallery_image_reference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inboundNatRules".into(),
                    value: &inbound_nat_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labName".into(),
                    value: &lab_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labSubnetName".into(),
                    value: &lab_subnet_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labVirtualNetworkId".into(),
                    value: &lab_virtual_network_id_binding.drop_type(),
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
                    name: "notes".into(),
                    value: &notes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "size".into(),
                    value: &size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshKey".into(),
                    value: &ssh_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: &username_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinuxVirtualMachineResult {
            id: o.get_field("id"),
            allow_claim: o.get_field("allowClaim"),
            disallow_public_ip_address: o.get_field("disallowPublicIpAddress"),
            fqdn: o.get_field("fqdn"),
            gallery_image_reference: o.get_field("galleryImageReference"),
            inbound_nat_rules: o.get_field("inboundNatRules"),
            lab_name: o.get_field("labName"),
            lab_subnet_name: o.get_field("labSubnetName"),
            lab_virtual_network_id: o.get_field("labVirtualNetworkId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            notes: o.get_field("notes"),
            password: o.get_field("password"),
            resource_group_name: o.get_field("resourceGroupName"),
            size: o.get_field("size"),
            ssh_key: o.get_field("sshKey"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            unique_identifier: o.get_field("uniqueIdentifier"),
            username: o.get_field("username"),
        }
    }
}
