/// Manages a Shared Image within a Shared Image Gallery.
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
///   exampleSharedImageGallery:
///     type: azure:compute:SharedImageGallery
///     name: example
///     properties:
///       name: example_image_gallery
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       description: Shared images and things.
///       tags:
///         Hello: There
///         World: Example
///   exampleSharedImage:
///     type: azure:compute:SharedImage
///     name: example
///     properties:
///       name: my-image
///       galleryName: ${exampleSharedImageGallery.name}
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       osType: Linux
///       identifier:
///         publisher: PublisherName
///         offer: OfferName
///         sku: ExampleSku
/// ```
///
/// ## Import
///
/// Shared Images can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/sharedImage:SharedImage image1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/galleries/gallery1/images/image1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedImageArgs {
        /// Specifies if the Shared Image supports Accelerated Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub accelerated_network_support_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// CPU architecture supported by an OS. Possible values are `x64` and `Arm64`. Defaults to `x64`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub architecture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines. Changing this forces a new resource to be created.
        ///
        /// > **Note:**: Only one of `trusted_launch_supported`, `trusted_launch_enabled`, `confidential_vm_supported` and `confidential_vm_enabled` can be specified.
        #[builder(into, default)]
        pub confidential_vm_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub confidential_vm_supported: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description of this Shared Image.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Shared Image supports NVMe disks. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub disk_controller_type_nvme_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// One or more Disk Types not allowed for the Image. Possible values include `Standard_LRS` and `Premium_LRS`.
        #[builder(into, default)]
        pub disk_types_not_alloweds: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The end of life date in RFC3339 format of the Image.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The End User Licence Agreement for the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eula: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Shared Image Gallery in which this Shared Image should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if the Shared Image supports hibernation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hibernation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on. Possible values are `V1` and `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hyper_v_generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identifier` block as defined below.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::SharedImageIdentifier,
        >,
        /// Specifies the supported Azure location where the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum memory in GB recommended for the Image.
        #[builder(into, default)]
        pub max_recommended_memory_in_gb: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Maximum count of vCPUs recommended for the Image.
        #[builder(into, default)]
        pub max_recommended_vcpu_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum memory in GB recommended for the Image.
        #[builder(into, default)]
        pub min_recommended_memory_in_gb: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Minimum count of vCPUs recommended for the Image.
        #[builder(into, default)]
        pub min_recommended_vcpu_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of Operating System present in this Shared Image. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub os_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI containing the Privacy Statement associated with this Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub privacy_statement_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `purchase_plan` block as defined below.
        #[builder(into, default)]
        pub purchase_plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes associated with this Shared Image.
        #[builder(into, default)]
        pub release_note_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run). Changing this forces a new resource to be created.
        ///
        /// !> **Note:** It's recommended to Generalize images where possible - Specialized Images reuse the same UUID internally within each Virtual Machine, which can have unintended side-effects.
        #[builder(into, default)]
        pub specialized: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the Shared Image.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub trusted_launch_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub trusted_launch_supported: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SharedImageResult {
        /// Specifies if the Shared Image supports Accelerated Network. Changing this forces a new resource to be created.
        pub accelerated_network_support_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// CPU architecture supported by an OS. Possible values are `x64` and `Arm64`. Defaults to `x64`. Changing this forces a new resource to be created.
        pub architecture: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines. Changing this forces a new resource to be created.
        ///
        /// > **Note:**: Only one of `trusted_launch_supported`, `trusted_launch_enabled`, `confidential_vm_supported` and `confidential_vm_enabled` can be specified.
        pub confidential_vm_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image. Changing this forces a new resource to be created.
        pub confidential_vm_supported: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A description of this Shared Image.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if the Shared Image supports NVMe disks. Changing this forces a new resource to be created.
        pub disk_controller_type_nvme_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more Disk Types not allowed for the Image. Possible values include `Standard_LRS` and `Premium_LRS`.
        pub disk_types_not_alloweds: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The end of life date in RFC3339 format of the Image.
        pub end_of_life_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The End User Licence Agreement for the Shared Image. Changing this forces a new resource to be created.
        pub eula: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Shared Image Gallery in which this Shared Image should exist. Changing this forces a new resource to be created.
        pub gallery_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the Shared Image supports hibernation. Changing this forces a new resource to be created.
        pub hibernation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on. Possible values are `V1` and `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        pub hyper_v_generation: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identifier` block as defined below.
        pub identifier: pulumi_gestalt_rust::Output<
            super::super::types::compute::SharedImageIdentifier,
        >,
        /// Specifies the supported Azure location where the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Maximum memory in GB recommended for the Image.
        pub max_recommended_memory_in_gb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Maximum count of vCPUs recommended for the Image.
        pub max_recommended_vcpu_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum memory in GB recommended for the Image.
        pub min_recommended_memory_in_gb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum count of vCPUs recommended for the Image.
        pub min_recommended_vcpu_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Shared Image. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of Operating System present in this Shared Image. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// The URI containing the Privacy Statement associated with this Shared Image. Changing this forces a new resource to be created.
        pub privacy_statement_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `purchase_plan` block as defined below.
        pub purchase_plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes associated with this Shared Image.
        pub release_note_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run). Changing this forces a new resource to be created.
        ///
        /// !> **Note:** It's recommended to Generalize images where possible - Specialized Images reuse the same UUID internally within each Virtual Machine, which can have unintended side-effects.
        pub specialized: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the Shared Image.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image. Changing this forces a new resource to be created.
        pub trusted_launch_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image. Changing this forces a new resource to be created.
        pub trusted_launch_supported: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedImageArgs,
    ) -> SharedImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accelerated_network_support_enabled_binding = args
            .accelerated_network_support_enabled
            .get_output(context);
        let architecture_binding = args.architecture.get_output(context);
        let confidential_vm_enabled_binding = args
            .confidential_vm_enabled
            .get_output(context);
        let confidential_vm_supported_binding = args
            .confidential_vm_supported
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let disk_controller_type_nvme_enabled_binding = args
            .disk_controller_type_nvme_enabled
            .get_output(context);
        let disk_types_not_alloweds_binding = args
            .disk_types_not_alloweds
            .get_output(context);
        let end_of_life_date_binding = args.end_of_life_date.get_output(context);
        let eula_binding = args.eula.get_output(context);
        let gallery_name_binding = args.gallery_name.get_output(context);
        let hibernation_enabled_binding = args.hibernation_enabled.get_output(context);
        let hyper_v_generation_binding = args.hyper_v_generation.get_output(context);
        let identifier_binding = args.identifier.get_output(context);
        let location_binding = args.location.get_output(context);
        let max_recommended_memory_in_gb_binding = args
            .max_recommended_memory_in_gb
            .get_output(context);
        let max_recommended_vcpu_count_binding = args
            .max_recommended_vcpu_count
            .get_output(context);
        let min_recommended_memory_in_gb_binding = args
            .min_recommended_memory_in_gb
            .get_output(context);
        let min_recommended_vcpu_count_binding = args
            .min_recommended_vcpu_count
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let os_type_binding = args.os_type.get_output(context);
        let privacy_statement_uri_binding = args
            .privacy_statement_uri
            .get_output(context);
        let purchase_plan_binding = args.purchase_plan.get_output(context);
        let release_note_uri_binding = args.release_note_uri.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let specialized_binding = args.specialized.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trusted_launch_enabled_binding = args
            .trusted_launch_enabled
            .get_output(context);
        let trusted_launch_supported_binding = args
            .trusted_launch_supported
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/sharedImage:SharedImage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceleratedNetworkSupportEnabled".into(),
                    value: &accelerated_network_support_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "architecture".into(),
                    value: &architecture_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confidentialVmEnabled".into(),
                    value: &confidential_vm_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confidentialVmSupported".into(),
                    value: &confidential_vm_supported_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskControllerTypeNvmeEnabled".into(),
                    value: &disk_controller_type_nvme_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskTypesNotAlloweds".into(),
                    value: &disk_types_not_alloweds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endOfLifeDate".into(),
                    value: &end_of_life_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eula".into(),
                    value: &eula_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryName".into(),
                    value: &gallery_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hibernationEnabled".into(),
                    value: &hibernation_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hyperVGeneration".into(),
                    value: &hyper_v_generation_binding.drop_type(),
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
                    name: "maxRecommendedMemoryInGb".into(),
                    value: &max_recommended_memory_in_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxRecommendedVcpuCount".into(),
                    value: &max_recommended_vcpu_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minRecommendedMemoryInGb".into(),
                    value: &min_recommended_memory_in_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minRecommendedVcpuCount".into(),
                    value: &min_recommended_vcpu_count_binding.drop_type(),
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
                    name: "privacyStatementUri".into(),
                    value: &privacy_statement_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purchasePlan".into(),
                    value: &purchase_plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseNoteUri".into(),
                    value: &release_note_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "specialized".into(),
                    value: &specialized_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedLaunchEnabled".into(),
                    value: &trusted_launch_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedLaunchSupported".into(),
                    value: &trusted_launch_supported_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedImageResult {
            accelerated_network_support_enabled: o
                .get_field("acceleratedNetworkSupportEnabled"),
            architecture: o.get_field("architecture"),
            confidential_vm_enabled: o.get_field("confidentialVmEnabled"),
            confidential_vm_supported: o.get_field("confidentialVmSupported"),
            description: o.get_field("description"),
            disk_controller_type_nvme_enabled: o
                .get_field("diskControllerTypeNvmeEnabled"),
            disk_types_not_alloweds: o.get_field("diskTypesNotAlloweds"),
            end_of_life_date: o.get_field("endOfLifeDate"),
            eula: o.get_field("eula"),
            gallery_name: o.get_field("galleryName"),
            hibernation_enabled: o.get_field("hibernationEnabled"),
            hyper_v_generation: o.get_field("hyperVGeneration"),
            identifier: o.get_field("identifier"),
            location: o.get_field("location"),
            max_recommended_memory_in_gb: o.get_field("maxRecommendedMemoryInGb"),
            max_recommended_vcpu_count: o.get_field("maxRecommendedVcpuCount"),
            min_recommended_memory_in_gb: o.get_field("minRecommendedMemoryInGb"),
            min_recommended_vcpu_count: o.get_field("minRecommendedVcpuCount"),
            name: o.get_field("name"),
            os_type: o.get_field("osType"),
            privacy_statement_uri: o.get_field("privacyStatementUri"),
            purchase_plan: o.get_field("purchasePlan"),
            release_note_uri: o.get_field("releaseNoteUri"),
            resource_group_name: o.get_field("resourceGroupName"),
            specialized: o.get_field("specialized"),
            tags: o.get_field("tags"),
            trusted_launch_enabled: o.get_field("trustedLaunchEnabled"),
            trusted_launch_supported: o.get_field("trustedLaunchSupported"),
        }
    }
}
