/// Manages a Mobile Network Slice.
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
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkSlice:
///     type: azure:mobile:NetworkSlice
///     name: example
///     properties:
///       name: example-mns
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       description: an example slice
///       singleNetworkSliceSelectionAssistanceInformation:
///         sliceServiceType: 1
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Slice can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkSlice:NetworkSlice example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/mobileNetworks/mobileNetwork1/slices/slice1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod network_slice {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSliceArgs {
        /// A description for this Mobile Network Slice.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the Azure Region where the Mobile Network Slice should exist. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of Mobile Network which the Mobile Network Slice belongs to. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name which should be used for this Mobile Network Slice. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A `single_network_slice_selection_assistance_information` block as defined below. Single-network slice selection assistance information (S-NSSAI). Unique at the scope of a mobile network.
        #[builder(into)]
        pub single_network_slice_selection_assistance_information: pulumi_gestalt_rust::Input<
            super::super::types::mobile::NetworkSliceSingleNetworkSliceSelectionAssistanceInformation,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Slice.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSliceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A description for this Mobile Network Slice.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Mobile Network Slice should exist. Changing this forces a new Mobile Network Slice to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of Mobile Network which the Mobile Network Slice belongs to. Changing this forces a new Mobile Network Slice to be created.
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Slice. Changing this forces a new Mobile Network Slice to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `single_network_slice_selection_assistance_information` block as defined below. Single-network slice selection assistance information (S-NSSAI). Unique at the scope of a mobile network.
        pub single_network_slice_selection_assistance_information: pulumi_gestalt_rust::Output<
            super::super::types::mobile::NetworkSliceSingleNetworkSliceSelectionAssistanceInformation,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Slice.
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
        args: NetworkSliceArgs,
    ) -> NetworkSliceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkSliceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> NetworkSliceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkSliceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> NetworkSliceResult {
        let description_binding = args.description.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let mobile_network_id_binding = args.mobile_network_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let single_network_slice_selection_assistance_information_binding = args
            .single_network_slice_selection_assistance_information
            .get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/networkSlice:NetworkSlice".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singleNetworkSliceSelectionAssistanceInformation".into(),
                    value: &single_network_slice_selection_assistance_information_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        NetworkSliceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            description: o.get_field("description"),
            location: o.get_field("location"),
            mobile_network_id: o.get_field("mobileNetworkId"),
            name: o.get_field("name"),
            single_network_slice_selection_assistance_information: o
                .get_field("singleNetworkSliceSelectionAssistanceInformation"),
            tags: o.get_field("tags"),
        }
    }
}
