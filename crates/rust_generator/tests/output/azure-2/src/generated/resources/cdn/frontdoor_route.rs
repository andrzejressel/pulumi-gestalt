/// Manages a Front Door (standard/premium) Route.
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
///   exampleZone:
///     type: azure:dns:Zone
///     name: example
///     properties:
///       name: example.com
///       resourceGroupName: ${example.name}
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       skuName: Standard_AzureFrontDoor
///   exampleFrontdoorOriginGroup:
///     type: azure:cdn:FrontdoorOriginGroup
///     name: example
///     properties:
///       name: example-originGroup
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       loadBalancing:
///         additionalLatencyInMilliseconds: 0
///         sampleSize: 16
///         successfulSamplesRequired: 3
///   exampleFrontdoorOrigin:
///     type: azure:cdn:FrontdoorOrigin
///     name: example
///     properties:
///       name: example-origin
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       enabled: true
///       certificateNameCheckEnabled: false
///       hostName: contoso.com
///       httpPort: 80
///       httpsPort: 443
///       originHostHeader: www.contoso.com
///       priority: 1
///       weight: 1
///   exampleFrontdoorEndpoint:
///     type: azure:cdn:FrontdoorEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///   exampleFrontdoorRuleSet:
///     type: azure:cdn:FrontdoorRuleSet
///     name: example
///     properties:
///       name: ExampleRuleSet
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///   contoso:
///     type: azure:cdn:FrontdoorCustomDomain
///     properties:
///       name: contoso-custom-domain
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       dnsZoneId: ${exampleZone.id}
///       hostName:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - contoso
///               - ${exampleZone.name}
///           return: result
///       tls:
///         certificateType: ManagedCertificate
///         minimumTlsVersion: TLS12
///   fabrikam:
///     type: azure:cdn:FrontdoorCustomDomain
///     properties:
///       name: fabrikam-custom-domain
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       dnsZoneId: ${exampleZone.id}
///       hostName:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - fabrikam
///               - ${exampleZone.name}
///           return: result
///       tls:
///         certificateType: ManagedCertificate
///         minimumTlsVersion: TLS12
///   exampleFrontdoorRoute:
///     type: azure:cdn:FrontdoorRoute
///     name: example
///     properties:
///       name: example-route
///       cdnFrontdoorEndpointId: ${exampleFrontdoorEndpoint.id}
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       cdnFrontdoorOriginIds:
///         - ${exampleFrontdoorOrigin.id}
///       cdnFrontdoorRuleSetIds:
///         - ${exampleFrontdoorRuleSet.id}
///       enabled: true
///       forwardingProtocol: HttpsOnly
///       httpsRedirectEnabled: true
///       patternsToMatches:
///         - /*
///       supportedProtocols:
///         - Http
///         - Https
///       cdnFrontdoorCustomDomainIds:
///         - ${contoso.id}
///         - ${fabrikam.id}
///       linkToDefaultDomain: false
///       cache:
///         queryStringCachingBehavior: IgnoreSpecifiedQueryStrings
///         queryStrings:
///           - account
///           - settings
///         compressionEnabled: true
///         contentTypesToCompresses:
///           - text/html
///           - text/javascript
///           - text/xml
///   contosoFrontdoorCustomDomainAssociation:
///     type: azure:cdn:FrontdoorCustomDomainAssociation
///     name: contoso
///     properties:
///       cdnFrontdoorCustomDomainId: ${contoso.id}
///       cdnFrontdoorRouteIds:
///         - ${exampleFrontdoorRoute.id}
///   fabrikamFrontdoorCustomDomainAssociation:
///     type: azure:cdn:FrontdoorCustomDomainAssociation
///     name: fabrikam
///     properties:
///       cdnFrontdoorCustomDomainId: ${fabrikam.id}
///       cdnFrontdoorRouteIds:
///         - ${exampleFrontdoorRoute.id}
/// ```
///
/// ## Import
///
/// Front Door Routes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorRoute:FrontdoorRoute example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/afdEndpoints/endpoint1/routes/route1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod frontdoor_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorRouteArgs {
        /// A `cache` block as defined below.
        ///
        /// > **NOTE:** To disable caching, do not provide the `cache` block in the configuration file.
        #[builder(into, default)]
        pub cache: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cdn::FrontdoorRouteCache>,
        >,
        /// The IDs of the Front Door Custom Domains which are associated with this Front Door Route.
        #[builder(into, default)]
        pub cdn_frontdoor_custom_domain_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The resource ID of the Front Door Endpoint where this Front Door Route should exist. Changing this forces a new Front Door Route to be created.
        #[builder(into)]
        pub cdn_frontdoor_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Front Door Origin Group where this Front Door Route should be created.
        #[builder(into)]
        pub cdn_frontdoor_origin_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more Front Door Origin resource IDs that this Front Door Route will link to.
        #[builder(into)]
        pub cdn_frontdoor_origin_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A directory path on the Front Door Origin that can be used to retrieve content (e.g. `contoso.cloudapp.net/originpath`).
        #[builder(into, default)]
        pub cdn_frontdoor_origin_path: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of the Front Door Rule Set IDs which should be assigned to this Front Door Route.
        #[builder(into, default)]
        pub cdn_frontdoor_rule_set_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Is this Front Door Route enabled? Possible values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Protocol that will be use when forwarding traffic to backends. Possible values are `HttpOnly`, `HttpsOnly` or `MatchRequest`. Defaults to `MatchRequest`.
        #[builder(into, default)]
        pub forwarding_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Automatically redirect HTTP traffic to HTTPS traffic? Possible values are `true` or `false`. Defaults to `true`.
        ///
        /// > **NOTE:** The `https_redirect_enabled` rule is the first rule that will be executed.
        #[builder(into, default)]
        pub https_redirect_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should this Front Door Route be linked to the default endpoint? Possible values include `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub link_to_default_domain: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Front Door Route. Valid values must begin with a letter or number, end with a letter or number and may only contain letters, numbers and hyphens with a maximum length of 90 characters. Changing this forces a new Front Door Route to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The route patterns of the rule.
        #[builder(into)]
        pub patterns_to_matches: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// One or more Protocols supported by this Front Door Route. Possible values are `Http` or `Https`.
        ///
        /// > **NOTE:** If `https_redirect_enabled` is set to `true` the `supported_protocols` field must contain both `Http` and `Https` values.
        #[builder(into)]
        pub supported_protocols: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct FrontdoorRouteResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `cache` block as defined below.
        ///
        /// > **NOTE:** To disable caching, do not provide the `cache` block in the configuration file.
        pub cache: pulumi_gestalt_rust::Output<
            Option<super::super::types::cdn::FrontdoorRouteCache>,
        >,
        /// The IDs of the Front Door Custom Domains which are associated with this Front Door Route.
        pub cdn_frontdoor_custom_domain_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The resource ID of the Front Door Endpoint where this Front Door Route should exist. Changing this forces a new Front Door Route to be created.
        pub cdn_frontdoor_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Front Door Origin Group where this Front Door Route should be created.
        pub cdn_frontdoor_origin_group_id: pulumi_gestalt_rust::Output<String>,
        /// One or more Front Door Origin resource IDs that this Front Door Route will link to.
        pub cdn_frontdoor_origin_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A directory path on the Front Door Origin that can be used to retrieve content (e.g. `contoso.cloudapp.net/originpath`).
        pub cdn_frontdoor_origin_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of the Front Door Rule Set IDs which should be assigned to this Front Door Route.
        pub cdn_frontdoor_rule_set_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Is this Front Door Route enabled? Possible values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Protocol that will be use when forwarding traffic to backends. Possible values are `HttpOnly`, `HttpsOnly` or `MatchRequest`. Defaults to `MatchRequest`.
        pub forwarding_protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// Automatically redirect HTTP traffic to HTTPS traffic? Possible values are `true` or `false`. Defaults to `true`.
        ///
        /// > **NOTE:** The `https_redirect_enabled` rule is the first rule that will be executed.
        pub https_redirect_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should this Front Door Route be linked to the default endpoint? Possible values include `true` or `false`. Defaults to `true`.
        pub link_to_default_domain: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Front Door Route. Valid values must begin with a letter or number, end with a letter or number and may only contain letters, numbers and hyphens with a maximum length of 90 characters. Changing this forces a new Front Door Route to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The route patterns of the rule.
        pub patterns_to_matches: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more Protocols supported by this Front Door Route. Possible values are `Http` or `Https`.
        ///
        /// > **NOTE:** If `https_redirect_enabled` is set to `true` the `supported_protocols` field must contain both `Http` and `Https` values.
        pub supported_protocols: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FrontdoorRouteArgs,
    ) -> FrontdoorRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cache_binding = args.cache.get_output(context);
        let cdn_frontdoor_custom_domain_ids_binding = args
            .cdn_frontdoor_custom_domain_ids
            .get_output(context);
        let cdn_frontdoor_endpoint_id_binding = args
            .cdn_frontdoor_endpoint_id
            .get_output(context);
        let cdn_frontdoor_origin_group_id_binding = args
            .cdn_frontdoor_origin_group_id
            .get_output(context);
        let cdn_frontdoor_origin_ids_binding = args
            .cdn_frontdoor_origin_ids
            .get_output(context);
        let cdn_frontdoor_origin_path_binding = args
            .cdn_frontdoor_origin_path
            .get_output(context);
        let cdn_frontdoor_rule_set_ids_binding = args
            .cdn_frontdoor_rule_set_ids
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let forwarding_protocol_binding = args.forwarding_protocol.get_output(context);
        let https_redirect_enabled_binding = args
            .https_redirect_enabled
            .get_output(context);
        let link_to_default_domain_binding = args
            .link_to_default_domain
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let patterns_to_matches_binding = args.patterns_to_matches.get_output(context);
        let supported_protocols_binding = args.supported_protocols.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorRoute:FrontdoorRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cache".into(),
                    value: &cache_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorCustomDomainIds".into(),
                    value: &cdn_frontdoor_custom_domain_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorEndpointId".into(),
                    value: &cdn_frontdoor_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorOriginGroupId".into(),
                    value: &cdn_frontdoor_origin_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorOriginIds".into(),
                    value: &cdn_frontdoor_origin_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorOriginPath".into(),
                    value: &cdn_frontdoor_origin_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorRuleSetIds".into(),
                    value: &cdn_frontdoor_rule_set_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardingProtocol".into(),
                    value: &forwarding_protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsRedirectEnabled".into(),
                    value: &https_redirect_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkToDefaultDomain".into(),
                    value: &link_to_default_domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patternsToMatches".into(),
                    value: &patterns_to_matches_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedProtocols".into(),
                    value: &supported_protocols_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FrontdoorRouteResult {
            id: o.get_field("id"),
            cache: o.get_field("cache"),
            cdn_frontdoor_custom_domain_ids: o.get_field("cdnFrontdoorCustomDomainIds"),
            cdn_frontdoor_endpoint_id: o.get_field("cdnFrontdoorEndpointId"),
            cdn_frontdoor_origin_group_id: o.get_field("cdnFrontdoorOriginGroupId"),
            cdn_frontdoor_origin_ids: o.get_field("cdnFrontdoorOriginIds"),
            cdn_frontdoor_origin_path: o.get_field("cdnFrontdoorOriginPath"),
            cdn_frontdoor_rule_set_ids: o.get_field("cdnFrontdoorRuleSetIds"),
            enabled: o.get_field("enabled"),
            forwarding_protocol: o.get_field("forwardingProtocol"),
            https_redirect_enabled: o.get_field("httpsRedirectEnabled"),
            link_to_default_domain: o.get_field("linkToDefaultDomain"),
            name: o.get_field("name"),
            patterns_to_matches: o.get_field("patternsToMatches"),
            supported_protocols: o.get_field("supportedProtocols"),
        }
    }
}
