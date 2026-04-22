/// Manages an SAP Discovery Virtual Instance.
///
/// > **Note:** Before using this resource, it's required to submit the request of registering the Resource Provider with Azure CLI `az provider register --namespace "Microsoft.Workloads"`. The Resource Provider can take a while to register, you can check the status by running `az provider show --namespace "Microsoft.Workloads" --query "registrationState"`. Once this outputs "Registered" the Resource Provider is available for use.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-sapvis
///       location: West Europe
///   exampleDiscoveryVirtualInstance:
///     type: azure:workloadssap:DiscoveryVirtualInstance
///     name: example
///     properties:
///       name: X01
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       environment: NonProd
///       sapProduct: S4HANA
///       centralServerVirtualMachineId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleRG/providers/Microsoft.Compute/virtualMachines/csvm1
///       managedStorageAccountName: managedsa
///       identity:
///         type: UserAssigned
///         identityIds:
///           - /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleRG/providers/Microsoft.ManagedIdentity/userAssignedIdentities/uai1
/// ```
///
/// ## Import
///
/// SAP Discovery Virtual Instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:workloadssap/discoveryVirtualInstance:DiscoveryVirtualInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Workloads/sapVirtualInstances/vis1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod discovery_virtual_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiscoveryVirtualInstanceArgs {
        /// The ID of the Virtual Machine of the Central Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub central_server_virtual_machine_id: pulumi_gestalt_rust::Input<String>,
        /// The environment type for the SAP Discovery Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::Input<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::Input<
            Option<super::super::types::workloadssap::DiscoveryVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the managed Resource Group for the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the custom Storage Account created by the service in the managed Resource Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_storage_account_name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name of the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The SAP Product type for the SAP Discovery Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sap_product: pulumi_gestalt_rust::Input<String>,
        /// A mapping of tags which should be assigned to the SAP Discovery Virtual Instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DiscoveryVirtualInstanceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Machine of the Central Server. Changing this forces a new resource to be created.
        pub central_server_virtual_machine_id: pulumi_gestalt_rust::Output<String>,
        /// The environment type for the SAP Discovery Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::workloadssap::DiscoveryVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the managed Resource Group for the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the custom Storage Account created by the service in the managed Resource Group. Changing this forces a new resource to be created.
        pub managed_storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SAP Product type for the SAP Discovery Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        pub sap_product: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAP Discovery Virtual Instance.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiscoveryVirtualInstanceArgs,
    ) -> DiscoveryVirtualInstanceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiscoveryVirtualInstanceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DiscoveryVirtualInstanceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiscoveryVirtualInstanceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DiscoveryVirtualInstanceResult {
        let central_server_virtual_machine_id_binding = args
            .central_server_virtual_machine_id
            .get_output(ctx);
        let environment_binding = args.environment.get_output(ctx);
        let identity_binding = args.identity.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_output(ctx);
        let managed_storage_account_name_binding = args
            .managed_storage_account_name
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let sap_product_binding = args.sap_product.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:workloadssap/discoveryVirtualInstance:DiscoveryVirtualInstance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "centralServerVirtualMachineId".into(),
                    value: &central_server_virtual_machine_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: &environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedStorageAccountName".into(),
                    value: &managed_storage_account_name_binding.drop_type(),
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
                    name: "sapProduct".into(),
                    value: &sap_product_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DiscoveryVirtualInstanceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            central_server_virtual_machine_id: o
                .get_field("centralServerVirtualMachineId"),
            environment: o.get_field("environment"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            managed_resource_group_name: o.get_field("managedResourceGroupName"),
            managed_storage_account_name: o.get_field("managedStorageAccountName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sap_product: o.get_field("sapProduct"),
            tags: o.get_field("tags"),
        }
    }
}
