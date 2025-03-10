/// VPN tunnel resource.
///
///
/// To get more information about VpnTunnel, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/vpnTunnels)
/// * How-to Guides
///     * [Cloud VPN Overview](https://cloud.google.com/vpn/docs/concepts/overview)
///     * [Networks and Tunnel Routing](https://cloud.google.com/vpn/docs/concepts/choosing-networks-routing)
///
///
///
/// ## Example Usage
///
/// ### Vpn Tunnel Basic
///
///
/// ```yaml
/// resources:
///   tunnel1:
///     type: gcp:compute:VPNTunnel
///     properties:
///       name: tunnel-1
///       peerIp: 15.0.0.120
///       sharedSecret: a secret message
///       targetVpnGateway: ${targetGateway.id}
///       labels:
///         foo: bar
///     options:
///       dependsOn:
///         - ${frEsp}
///         - ${frUdp500}
///         - ${frUdp4500}
///   targetGateway:
///     type: gcp:compute:VPNGateway
///     name: target_gateway
///     properties:
///       name: vpn-1
///       network: ${network1.id}
///   network1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///   vpnStaticIp:
///     type: gcp:compute:Address
///     name: vpn_static_ip
///     properties:
///       name: vpn-static-ip
///   frEsp:
///     type: gcp:compute:ForwardingRule
///     name: fr_esp
///     properties:
///       name: fr-esp
///       ipProtocol: ESP
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   frUdp500:
///     type: gcp:compute:ForwardingRule
///     name: fr_udp500
///     properties:
///       name: fr-udp500
///       ipProtocol: UDP
///       portRange: '500'
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   frUdp4500:
///     type: gcp:compute:ForwardingRule
///     name: fr_udp4500
///     properties:
///       name: fr-udp4500
///       ipProtocol: UDP
///       portRange: '4500'
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   route1:
///     type: gcp:compute:Route
///     properties:
///       name: route1
///       network: ${network1.name}
///       destRange: 15.0.0.0/24
///       priority: 1000
///       nextHopVpnTunnel: ${tunnel1.id}
/// ```
///
/// ## Import
///
/// VpnTunnel can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/vpnTunnels/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, VpnTunnel can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/vPNTunnel:VPNTunnel default projects/{{project}}/regions/{{region}}/vpnTunnels/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/vPNTunnel:VPNTunnel default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/vPNTunnel:VPNTunnel default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/vPNTunnel:VPNTunnel default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_tunnel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VPNTunnelArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IKE protocol version to use when establishing the VPN tunnel with
        /// peer VPN gateway.
        /// Acceptable IKE versions are 1 or 2. Default version is 2.
        #[builder(into, default)]
        pub ike_version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Labels to apply to this VpnTunnel.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Local traffic selector to use when establishing the VPN tunnel with
        /// peer VPN gateway. The value should be a CIDR formatted string,
        /// for example `192.168.0.0/16`. The ranges should be disjoint.
        /// Only IPv4 is supported.
        #[builder(into, default)]
        pub local_traffic_selectors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63
        /// characters long and match the regular expression
        /// `a-z?` which means the first character
        /// must be a lowercase letter, and all following characters must
        /// be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the peer side external VPN gateway to which this VPN tunnel is connected.
        #[builder(into, default)]
        pub peer_external_gateway: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The interface ID of the external VPN gateway to which this VPN tunnel is connected.
        #[builder(into, default)]
        pub peer_external_gateway_interface: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// URL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.
        /// If provided, the VPN tunnel will automatically use the same vpn_gateway_interface
        /// ID in the peer GCP VPN gateway.
        /// This field must reference a `gcp.compute.HaVpnGateway` resource.
        #[builder(into, default)]
        pub peer_gcp_gateway: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP address of the peer VPN gateway. Only IPv4 is supported.
        #[builder(into, default)]
        pub peer_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region where the tunnel is located. If unset, is set to the region of `target_vpn_gateway`.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Remote traffic selector to use when establishing the VPN tunnel with
        /// peer VPN gateway. The value should be a CIDR formatted string,
        /// for example `192.168.0.0/16`. The ranges should be disjoint.
        /// Only IPv4 is supported.
        #[builder(into, default)]
        pub remote_traffic_selectors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// URL of router resource to be used for dynamic routing.
        #[builder(into, default)]
        pub router: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Shared secret used to set the secure session between the Cloud VPN
        /// gateway and the peer VPN gateway.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub shared_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URL of the Target VPN gateway with which this VPN tunnel is
        /// associated.
        #[builder(into, default)]
        pub target_vpn_gateway: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the VPN gateway with which this VPN tunnel is associated.
        /// This must be used if a High Availability VPN gateway resource is created.
        /// This field must reference a `gcp.compute.HaVpnGateway` resource.
        #[builder(into, default)]
        pub vpn_gateway: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The interface ID of the VPN gateway with which this VPN tunnel is associated.
        #[builder(into, default)]
        pub vpn_gateway_interface: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VPNTunnelResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Detailed status message for the VPN tunnel.
        pub detailed_status: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// IKE protocol version to use when establishing the VPN tunnel with
        /// peer VPN gateway.
        /// Acceptable IKE versions are 1 or 2. Default version is 2.
        pub ike_version: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels to apply to this VpnTunnel.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Local traffic selector to use when establishing the VPN tunnel with
        /// peer VPN gateway. The value should be a CIDR formatted string,
        /// for example `192.168.0.0/16`. The ranges should be disjoint.
        /// Only IPv4 is supported.
        pub local_traffic_selectors: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63
        /// characters long and match the regular expression
        /// `a-z?` which means the first character
        /// must be a lowercase letter, and all following characters must
        /// be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// URL of the peer side external VPN gateway to which this VPN tunnel is connected.
        pub peer_external_gateway: pulumi_gestalt_rust::Output<Option<String>>,
        /// The interface ID of the external VPN gateway to which this VPN tunnel is connected.
        pub peer_external_gateway_interface: pulumi_gestalt_rust::Output<Option<i32>>,
        /// URL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.
        /// If provided, the VPN tunnel will automatically use the same vpn_gateway_interface
        /// ID in the peer GCP VPN gateway.
        /// This field must reference a `gcp.compute.HaVpnGateway` resource.
        pub peer_gcp_gateway: pulumi_gestalt_rust::Output<Option<String>>,
        /// IP address of the peer VPN gateway. Only IPv4 is supported.
        pub peer_ip: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region where the tunnel is located. If unset, is set to the region of `target_vpn_gateway`.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Remote traffic selector to use when establishing the VPN tunnel with
        /// peer VPN gateway. The value should be a CIDR formatted string,
        /// for example `192.168.0.0/16`. The ranges should be disjoint.
        /// Only IPv4 is supported.
        pub remote_traffic_selectors: pulumi_gestalt_rust::Output<Vec<String>>,
        /// URL of router resource to be used for dynamic routing.
        pub router: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Shared secret used to set the secure session between the Cloud VPN
        /// gateway and the peer VPN gateway.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        ///
        ///
        /// - - -
        pub shared_secret: pulumi_gestalt_rust::Output<String>,
        /// Hash of the shared secret.
        pub shared_secret_hash: pulumi_gestalt_rust::Output<String>,
        /// URL of the Target VPN gateway with which this VPN tunnel is
        /// associated.
        pub target_vpn_gateway: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub tunnel_id: pulumi_gestalt_rust::Output<String>,
        /// URL of the VPN gateway with which this VPN tunnel is associated.
        /// This must be used if a High Availability VPN gateway resource is created.
        /// This field must reference a `gcp.compute.HaVpnGateway` resource.
        pub vpn_gateway: pulumi_gestalt_rust::Output<Option<String>>,
        /// The interface ID of the VPN gateway with which this VPN tunnel is associated.
        pub vpn_gateway_interface: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VPNTunnelArgs,
    ) -> VPNTunnelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let ike_version_binding = args.ike_version.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let local_traffic_selectors_binding = args
            .local_traffic_selectors
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let peer_external_gateway_binding = args
            .peer_external_gateway
            .get_output(context);
        let peer_external_gateway_interface_binding = args
            .peer_external_gateway_interface
            .get_output(context);
        let peer_gcp_gateway_binding = args.peer_gcp_gateway.get_output(context);
        let peer_ip_binding = args.peer_ip.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let remote_traffic_selectors_binding = args
            .remote_traffic_selectors
            .get_output(context);
        let router_binding = args.router.get_output(context);
        let shared_secret_binding = args.shared_secret.get_output(context);
        let target_vpn_gateway_binding = args.target_vpn_gateway.get_output(context);
        let vpn_gateway_binding = args.vpn_gateway.get_output(context);
        let vpn_gateway_interface_binding = args
            .vpn_gateway_interface
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/vPNTunnel:VPNTunnel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ikeVersion".into(),
                    value: &ike_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localTrafficSelectors".into(),
                    value: &local_traffic_selectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerExternalGateway".into(),
                    value: &peer_external_gateway_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerExternalGatewayInterface".into(),
                    value: &peer_external_gateway_interface_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerGcpGateway".into(),
                    value: &peer_gcp_gateway_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerIp".into(),
                    value: &peer_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteTrafficSelectors".into(),
                    value: &remote_traffic_selectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "router".into(),
                    value: &router_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedSecret".into(),
                    value: &shared_secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVpnGateway".into(),
                    value: &target_vpn_gateway_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnGateway".into(),
                    value: &vpn_gateway_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnGatewayInterface".into(),
                    value: &vpn_gateway_interface_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VPNTunnelResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            detailed_status: o.get_field("detailedStatus"),
            effective_labels: o.get_field("effectiveLabels"),
            ike_version: o.get_field("ikeVersion"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            local_traffic_selectors: o.get_field("localTrafficSelectors"),
            name: o.get_field("name"),
            peer_external_gateway: o.get_field("peerExternalGateway"),
            peer_external_gateway_interface: o.get_field("peerExternalGatewayInterface"),
            peer_gcp_gateway: o.get_field("peerGcpGateway"),
            peer_ip: o.get_field("peerIp"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            remote_traffic_selectors: o.get_field("remoteTrafficSelectors"),
            router: o.get_field("router"),
            self_link: o.get_field("selfLink"),
            shared_secret: o.get_field("sharedSecret"),
            shared_secret_hash: o.get_field("sharedSecretHash"),
            target_vpn_gateway: o.get_field("targetVpnGateway"),
            tunnel_id: o.get_field("tunnelId"),
            vpn_gateway: o.get_field("vpnGateway"),
            vpn_gateway_interface: o.get_field("vpnGatewayInterface"),
        }
    }
}
