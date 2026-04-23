/// Manages an Azure Stack HCI Marketplace Gallery Image.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: examples
///       location: West Europe
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleResourceGroup.id}
///       roleDefinitionName: Azure Connected Machine Resource Manager
///       principalId: ${hciRp.objectId}
///   exampleHciMarketplaceGalleryImage:
///     type: azure:stack:HciMarketplaceGalleryImage
///     name: example
///     properties:
///       name: example-mgi
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       hypervGeneration: V2
///       osType: Windows
///       version: 20348.2655.240905
///       identifier:
///         publisher: MicrosoftWindowsServer
///         offer: WindowsServer
///         sku: 2022-datacenter-azure-edition-core
///       tags:
///         foo: bar
///         env: example
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   # service principal of 'Microsoft.AzureStackHCI Resource Provider'
///   hciRp:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         clientId: 1412d89f-b8a8-4111-b4fd-e82905cbd85d
/// ```
///
/// ## Import
///
/// Azure Stack HCI Marketplace Gallery Images can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciMarketplaceGalleryImage:HciMarketplaceGalleryImage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.AzureStackHCI/marketplaceGalleryImages/image1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod hci_marketplace_gallery_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciMarketplaceGalleryImageArgs {
        /// The ID of the Custom Location where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_gestalt_rust::Input<String>,
        /// The hypervisor generation of the Azure Stack HCI Marketplace Gallery Image. Possible values are `V1` and `V2`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub hyperv_generation: pulumi_gestalt_rust::Input<String>,
        /// An `identifier` block as defined below. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::Input<
            super::super::types::stack::HciMarketplaceGalleryImageIdentifier,
        >,
        /// The Azure Region where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The Operating System type of the Azure Stack HCI Marketplace Gallery Image. Possible values are `Windows` and `Linux`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub os_type: pulumi_gestalt_rust::Input<String>,
        /// The name of the Resource Group where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The ID of the Azure Stack HCI Storage Path used for this Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub storage_path_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Marketplace Gallery Image.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct HciMarketplaceGalleryImageResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Custom Location where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_gestalt_rust::Output<String>,
        /// The hypervisor generation of the Azure Stack HCI Marketplace Gallery Image. Possible values are `V1` and `V2`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub hyperv_generation: pulumi_gestalt_rust::Output<String>,
        /// An `identifier` block as defined below. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub identifier: pulumi_gestalt_rust::Output<
            super::super::types::stack::HciMarketplaceGalleryImageIdentifier,
        >,
        /// The Azure Region where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Operating System type of the Azure Stack HCI Marketplace Gallery Image. Possible values are `Windows` and `Linux`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure Stack HCI Storage Path used for this Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub storage_path_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Marketplace Gallery Image.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciMarketplaceGalleryImageArgs,
    ) -> HciMarketplaceGalleryImageResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciMarketplaceGalleryImageArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> HciMarketplaceGalleryImageResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciMarketplaceGalleryImageArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> HciMarketplaceGalleryImageResult {
        let custom_location_id_binding = args.custom_location_id.get_output(ctx);
        let hyperv_generation_binding = args.hyperv_generation.get_output(ctx);
        let identifier_binding = args.identifier.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let os_type_binding = args.os_type.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let storage_path_id_binding = args.storage_path_id.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let version_binding = args.version.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:stack/hciMarketplaceGalleryImage:HciMarketplaceGalleryImage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hypervGeneration".into(),
                    value: &hyperv_generation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding.drop_type(),
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
                    name: "osType".into(),
                    value: &os_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storagePathId".into(),
                    value: &storage_path_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        HciMarketplaceGalleryImageResult {
            id: o.get_id(),
            urn: o.get_urn(),
            custom_location_id: o.get_field("customLocationId"),
            hyperv_generation: o.get_field("hypervGeneration"),
            identifier: o.get_field("identifier"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            os_type: o.get_field("osType"),
            resource_group_name: o.get_field("resourceGroupName"),
            storage_path_id: o.get_field("storagePathId"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
        }
    }
}
