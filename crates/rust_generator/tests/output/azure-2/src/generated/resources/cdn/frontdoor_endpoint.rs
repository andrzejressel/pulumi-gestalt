/// Manages a Front Door (standard/premium) Endpoint.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-cdn-frontdoor
///       location: West Europe
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       skuName: Standard_AzureFrontDoor
///   exampleFrontdoorEndpoint:
///     type: azure:cdn:FrontdoorEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       tags:
///         ENV: example
/// ```
///
/// ## Import
///
/// Front Door Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorEndpoint:FrontdoorEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/afdEndpoints/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod frontdoor_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorEndpointArgs {
        /// The ID of the Front Door Profile within which this Front Door Endpoint should exist. Changing this forces a new Front Door Endpoint to be created.
        #[builder(into)]
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if this Front Door Endpoint is enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Front Door Endpoint. Changing this forces a new Front Door Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a mapping of tags which should be assigned to the Front Door Endpoint.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorEndpointResult {
        /// The ID of the Front Door Profile within which this Front Door Endpoint should exist. Changing this forces a new Front Door Endpoint to be created.
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies if this Front Door Endpoint is enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The host name of the Front Door Endpoint, in the format `{endpointName}.{dnsZone}` (for example, `contoso.azureedge.net`).
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Front Door Endpoint. Changing this forces a new Front Door Endpoint to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a mapping of tags which should be assigned to the Front Door Endpoint.
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
        args: FrontdoorEndpointArgs,
    ) -> FrontdoorEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cdn_frontdoor_profile_id_binding = args
            .cdn_frontdoor_profile_id
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorEndpoint:FrontdoorEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorProfileId".into(),
                    value: &cdn_frontdoor_profile_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FrontdoorEndpointResult {
            cdn_frontdoor_profile_id: o.get_field("cdnFrontdoorProfileId"),
            enabled: o.get_field("enabled"),
            host_name: o.get_field("hostName"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
