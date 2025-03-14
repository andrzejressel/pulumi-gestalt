#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_environment_v_3 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentV3Args {
        /// The name of this v3 App Service Environment.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the v3 App Service Environment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentV3Result {
        /// Are new Private Endpoint Connections allowed.
        pub allow_new_private_endpoint_connections: pulumi_gestalt_rust::Output<bool>,
        /// A `cluster_setting` block as defined below.
        pub cluster_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetEnvironmentV3ClusterSetting>,
        >,
        /// The number of Dedicated Hosts used by this ASEv3.
        pub dedicated_host_count: pulumi_gestalt_rust::Output<i32>,
        /// the DNS suffix for this App Service Environment V3.
        pub dns_suffix: pulumi_gestalt_rust::Output<String>,
        /// The external inbound IP addresses of the App Service Environment V3.
        pub external_inbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An Inbound Network Dependencies block as defined below.
        pub inbound_network_dependencies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appservice::GetEnvironmentV3InboundNetworkDependency,
            >,
        >,
        /// The internal inbound IP addresses of the App Service Environment V3.
        pub internal_inbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Internal Load Balancing Mode of this ASEv3.
        pub internal_load_balancing_mode: pulumi_gestalt_rust::Output<String>,
        /// The number of IP SSL addresses reserved for the App Service Environment V3.
        pub ip_ssl_address_count: pulumi_gestalt_rust::Output<i32>,
        /// The list of Outbound IP Addresses of Linux based Apps in this App Service Environment V3.
        pub linux_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The location where the App Service Environment exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cluster Setting.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Pricing tier for the front end instances.
        pub pricing_tier: pulumi_gestalt_rust::Output<String>,
        pub remote_debugging_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the v3 App Service Environment Subnet.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the v3 App Service Environment.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Outbound addresses of Windows based Apps in this App Service Environment V3.
        pub windows_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub zone_redundant: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEnvironmentV3Args,
    ) -> GetEnvironmentV3Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getEnvironmentV3:getEnvironmentV3".into(),
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
        GetEnvironmentV3Result {
            allow_new_private_endpoint_connections: o
                .get_field("allowNewPrivateEndpointConnections"),
            cluster_settings: o.get_field("clusterSettings"),
            dedicated_host_count: o.get_field("dedicatedHostCount"),
            dns_suffix: o.get_field("dnsSuffix"),
            external_inbound_ip_addresses: o.get_field("externalInboundIpAddresses"),
            id: o.get_field("id"),
            inbound_network_dependencies: o.get_field("inboundNetworkDependencies"),
            internal_inbound_ip_addresses: o.get_field("internalInboundIpAddresses"),
            internal_load_balancing_mode: o.get_field("internalLoadBalancingMode"),
            ip_ssl_address_count: o.get_field("ipSslAddressCount"),
            linux_outbound_ip_addresses: o.get_field("linuxOutboundIpAddresses"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pricing_tier: o.get_field("pricingTier"),
            remote_debugging_enabled: o.get_field("remoteDebuggingEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            windows_outbound_ip_addresses: o.get_field("windowsOutboundIpAddresses"),
            zone_redundant: o.get_field("zoneRedundant"),
        }
    }
}
