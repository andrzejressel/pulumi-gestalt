/// A VPN connection
///
///
/// To get more information about VpnConnection, see:
///
/// * [API documentation](https://cloud.google.com/distributed-cloud/edge/latest/docs/reference/container/rest/v1/projects.locations.vpnConnections)
/// * How-to Guides
///     * [Google Distributed Cloud Edge](https://cloud.google.com/distributed-cloud/edge/latest/docs)
///
/// ## Example Usage
///
/// ### Edgecontainer Vpn Connection
///
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: default
///       location: us-central1
///       authorization:
///         adminUsers:
///           username: admin@hashicorptest.com
///       networking:
///         clusterIpv4CidrBlocks:
///           - 10.0.0.0/16
///         servicesIpv4CidrBlocks:
///           - 10.1.0.0/16
///       fleet:
///         project: projects/${project.number}
///   nodePool:
///     type: gcp:edgecontainer:NodePool
///     name: node_pool
///     properties:
///       name: nodepool-1
///       cluster: ${cluster.name}
///       location: us-central1
///       nodeLocation: us-central1-edge-example-edgesite
///       nodeCount: 3
///   default:
///     type: gcp:edgecontainer:VpnConnection
///     properties:
///       name: vpn-connection-1
///       location: us-central1
///       cluster: projects/${project.number}/locations/us-east1/clusters/${cluster.name}
///       vpc: ${vpc.name}
///       enableHighAvailability: true
///       labels:
///         my_key: my_val
///         other_key: other_val
///     options:
///       dependsOn:
///         - ${nodePool}
///   vpc:
///     type: gcp:compute:Network
///     properties:
///       name: example-vpc
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// VpnConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vpnConnections/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, VpnConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/vpnConnection:VpnConnection default projects/{{project}}/locations/{{location}}/vpnConnections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/vpnConnection:VpnConnection default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/vpnConnection:VpnConnection default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnConnectionArgs {
        /// The canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}.
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs.
        #[builder(into, default)]
        pub enable_high_availability: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Labels associated with this resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Google Cloud Platform location.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of VPN connection
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// NAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.
        /// This is empty if NAT is not used.
        #[builder(into, default)]
        pub nat_gateway_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The VPN connection Cloud Router name.
        #[builder(into, default)]
        pub router: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network ID of VPC to connect to.
        #[builder(into, default)]
        pub vpc: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Project detail of the VPC network. Required if VPC is in a different project than the cluster project.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vpc_project: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::VpnConnectionVpcProject>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpnConnectionResult {
        /// The canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}.
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// The time when the VPN connection was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::edgecontainer::VpnConnectionDetail>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs.
        pub enable_high_availability: pulumi_gestalt_rust::Output<bool>,
        /// Labels associated with this resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Google Cloud Platform location.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of VPN connection
        pub name: pulumi_gestalt_rust::Output<String>,
        /// NAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.
        /// This is empty if NAT is not used.
        pub nat_gateway_ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPN connection Cloud Router name.
        pub router: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time when the VPN connection was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The network ID of VPC to connect to.
        pub vpc: pulumi_gestalt_rust::Output<Option<String>>,
        /// Project detail of the VPC network. Required if VPC is in a different project than the cluster project.
        /// Structure is documented below.
        pub vpc_project: pulumi_gestalt_rust::Output<
            Option<super::super::types::edgecontainer::VpnConnectionVpcProject>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnConnectionArgs,
    ) -> VpnConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_binding = args.cluster.get_output(context);
        let enable_high_availability_binding = args
            .enable_high_availability
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let nat_gateway_ip_binding = args.nat_gateway_ip.get_output(context);
        let project_binding = args.project.get_output(context);
        let router_binding = args.router.get_output(context);
        let vpc_binding = args.vpc.get_output(context);
        let vpc_project_binding = args.vpc_project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:edgecontainer/vpnConnection:VpnConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHighAvailability".into(),
                    value: &enable_high_availability_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
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
                    name: "natGatewayIp".into(),
                    value: &nat_gateway_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "router".into(),
                    value: &router_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcProject".into(),
                    value: &vpc_project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpnConnectionResult {
            cluster: o.get_field("cluster"),
            create_time: o.get_field("createTime"),
            details: o.get_field("details"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_high_availability: o.get_field("enableHighAvailability"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            nat_gateway_ip: o.get_field("natGatewayIp"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            router: o.get_field("router"),
            update_time: o.get_field("updateTime"),
            vpc: o.get_field("vpc"),
            vpc_project: o.get_field("vpcProject"),
        }
    }
}
