#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_application_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationGatewayArgs {
        /// The name of this Application Gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Application Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApplicationGatewayResult {
        /// One or more `authentication_certificate` blocks as defined below.
        pub authentication_certificates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayAuthenticationCertificate,
            >,
        >,
        /// An `autoscale_configuration` block as defined below.
        pub autoscale_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayAutoscaleConfiguration,
            >,
        >,
        /// One or more `backend_address_pool` blocks as defined below.
        pub backend_address_pools: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayBackendAddressPool,
            >,
        >,
        /// One or more `backend_http_settings` blocks as defined below.
        pub backend_http_settings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayBackendHttpSetting,
            >,
        >,
        /// One or more `custom_error_configuration` blocks as defined below.
        pub custom_error_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayCustomErrorConfiguration,
            >,
        >,
        /// Is FIPS enabled on the Application Gateway?
        pub fips_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Web Application Firewall Policy which is used as an HTTP Listener for this Path Rule.
        pub firewall_policy_id: pulumi_gestalt_rust::Output<String>,
        /// Is the Firewall Policy associated with the Application Gateway?
        pub force_firewall_policy_association: pulumi_gestalt_rust::Output<bool>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        pub frontend_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayFrontendIpConfiguration,
            >,
        >,
        /// One or more `frontend_port` blocks as defined below.
        pub frontend_ports: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayFrontendPort>,
        >,
        /// One or more `gateway_ip_configuration` blocks as defined below.
        pub gateway_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayGatewayIpConfiguration,
            >,
        >,
        /// A `global` block as defined below.
        pub globals: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayGlobal>,
        >,
        /// Is HTTP2 enabled on the application gateway resource?
        pub http2_enabled: pulumi_gestalt_rust::Output<bool>,
        /// One or more `http_listener` blocks as defined below.
        pub http_listeners: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayHttpListener>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayIdentity>,
        >,
        /// The Azure region where the Application Gateway exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Unique name of the Rewrite Rule
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_endpoint_connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayPrivateEndpointConnection,
            >,
        >,
        /// One or more `private_link_configuration` blocks as defined below.
        pub private_link_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayPrivateLinkConfiguration,
            >,
        >,
        /// One or more `probe` blocks as defined below.
        pub probes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayProbe>,
        >,
        /// One or more `redirect_configuration` blocks as defined below.
        pub redirect_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayRedirectConfiguration,
            >,
        >,
        /// One or more `request_routing_rule` blocks as defined below.
        pub request_routing_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayRequestRoutingRule,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `rewrite_rule_set` blocks as defined below.
        pub rewrite_rule_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayRewriteRuleSet>,
        >,
        /// A `sku` block as defined below.
        pub skus: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySkus>,
        >,
        /// One or more `ssl_certificate` blocks as defined below.
        pub ssl_certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySslCertificate>,
        >,
        /// a `ssl_policy` block as defined below.
        pub ssl_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySslPolicy>,
        >,
        /// One or more `ssl_profile` blocks as defined below.
        pub ssl_profiles: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewaySslProfile>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// One or more `trusted_client_certificate` blocks as defined below.
        pub trusted_client_certificates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayTrustedClientCertificate,
            >,
        >,
        /// One or more `trusted_root_certificate` blocks as defined below.
        pub trusted_root_certificates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayTrustedRootCertificate,
            >,
        >,
        /// One or more `url_path_map` blocks as defined below.
        pub url_path_maps: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetApplicationGatewayUrlPathMap>,
        >,
        /// A `waf_configuration` block as defined below.
        pub waf_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetApplicationGatewayWafConfiguration,
            >,
        >,
        /// The list of Availability Zones in which this Application Gateway can use.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetApplicationGatewayArgs,
    ) -> GetApplicationGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getApplicationGateway:getApplicationGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetApplicationGatewayResult {
            authentication_certificates: o.get_field("authenticationCertificates"),
            autoscale_configurations: o.get_field("autoscaleConfigurations"),
            backend_address_pools: o.get_field("backendAddressPools"),
            backend_http_settings: o.get_field("backendHttpSettings"),
            custom_error_configurations: o.get_field("customErrorConfigurations"),
            fips_enabled: o.get_field("fipsEnabled"),
            firewall_policy_id: o.get_field("firewallPolicyId"),
            force_firewall_policy_association: o
                .get_field("forceFirewallPolicyAssociation"),
            frontend_ip_configurations: o.get_field("frontendIpConfigurations"),
            frontend_ports: o.get_field("frontendPorts"),
            gateway_ip_configurations: o.get_field("gatewayIpConfigurations"),
            globals: o.get_field("globals"),
            http2_enabled: o.get_field("http2Enabled"),
            http_listeners: o.get_field("httpListeners"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_endpoint_connections: o.get_field("privateEndpointConnections"),
            private_link_configurations: o.get_field("privateLinkConfigurations"),
            probes: o.get_field("probes"),
            redirect_configurations: o.get_field("redirectConfigurations"),
            request_routing_rules: o.get_field("requestRoutingRules"),
            resource_group_name: o.get_field("resourceGroupName"),
            rewrite_rule_sets: o.get_field("rewriteRuleSets"),
            skus: o.get_field("skus"),
            ssl_certificates: o.get_field("sslCertificates"),
            ssl_policies: o.get_field("sslPolicies"),
            ssl_profiles: o.get_field("sslProfiles"),
            tags: o.get_field("tags"),
            trusted_client_certificates: o.get_field("trustedClientCertificates"),
            trusted_root_certificates: o.get_field("trustedRootCertificates"),
            url_path_maps: o.get_field("urlPathMaps"),
            waf_configurations: o.get_field("wafConfigurations"),
            zones: o.get_field("zones"),
        }
    }
}
