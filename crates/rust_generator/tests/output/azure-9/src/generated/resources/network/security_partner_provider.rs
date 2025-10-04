/// Manages a Security Partner Provider which could be associated to virtual hub.
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
///   exampleVirtualWan:
///     type: azure:network:VirtualWan
///     name: example
///     properties:
///       name: example-vwan
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleVirtualHub:
///     type: azure:network:VirtualHub
///     name: example
///     properties:
///       name: example-vhub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       virtualWanId: ${exampleVirtualWan.id}
///       addressPrefix: 10.0.2.0/24
///   exampleVpnGateway:
///     type: azure:network:VpnGateway
///     name: example
///     properties:
///       name: example-vpngw
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       virtualHubId: ${exampleVirtualHub.id}
///   exampleSecurityPartnerProvider:
///     type: azure:network:SecurityPartnerProvider
///     name: example
///     properties:
///       name: example-spp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       virtualHubId: ${exampleVirtualHub.id}
///       securityProviderName: IBoss
///       tags:
///         ENV: Prod
///     options:
///       dependsOn:
///         - ${exampleVpnGateway}
/// ```
///
/// ## Import
///
/// Security Partner Providers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/securityPartnerProvider:SecurityPartnerProvider example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/securityPartnerProviders/securityPartnerProvider1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_partner_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityPartnerProviderArgs {
        /// The Azure Region where the Security Partner Provider should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Security Partner Provider. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Security Partner Provider should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The security provider name. Possible values are `ZScaler`, `IBoss` and `Checkpoint` is allowed. Changing this forces a new resource to be created.
        #[builder(into)]
        pub security_provider_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Security Partner Provider.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub within which this Security Partner Provider should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub virtual_hub_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityPartnerProviderResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Security Partner Provider should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Security Partner Provider. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Security Partner Provider should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The security provider name. Possible values are `ZScaler`, `IBoss` and `Checkpoint` is allowed. Changing this forces a new resource to be created.
        pub security_provider_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Security Partner Provider.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub within which this Security Partner Provider should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityPartnerProviderArgs,
    ) -> SecurityPartnerProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let security_provider_name_binding = args
            .security_provider_name
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/securityPartnerProvider:SecurityPartnerProvider"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
                    name: "securityProviderName".into(),
                    value: &security_provider_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityPartnerProviderResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            security_provider_name: o.get_field("securityProviderName"),
            tags: o.get_field("tags"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
