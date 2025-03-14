/// Represents a ServiceAttachment resource.
///
///
/// To get more information about ServiceAttachment, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/beta/serviceAttachments)
/// * How-to Guides
///     * [Configuring Private Service Connect to access services](https://cloud.google.com/vpc/docs/configure-private-service-connect-services)
///
/// ## Example Usage
///
/// ### Service Attachment Basic
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       domainNames:
///         - gcp.tfacc.hashicorptest.com.
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_AUTOMATIC
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///   pscIlbConsumerAddress:
///     type: gcp:compute:Address
///     name: psc_ilb_consumer_address
///     properties:
///       name: psc-ilb-consumer-address
///       region: us-west2
///       subnetwork: default
///       addressType: INTERNAL
///   pscIlbConsumer:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_consumer
///     properties:
///       name: psc-ilb-consumer-forwarding-rule
///       region: us-west2
///       target: ${pscIlbServiceAttachment.id}
///       loadBalancingScheme: ""
///       network: default
///       ipAddress: ${pscIlbConsumerAddress.id}
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
/// ### Service Attachment Explicit Projects
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       domainNames:
///         - gcp.tfacc.hashicorptest.com.
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_MANUAL
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///       consumerRejectLists:
///         - '673497134629'
///         - '482878270665'
///       consumerAcceptLists:
///         - projectIdOrNum: '658859330310'
///           connectionLimit: 4
///   pscIlbConsumerAddress:
///     type: gcp:compute:Address
///     name: psc_ilb_consumer_address
///     properties:
///       name: psc-ilb-consumer-address
///       region: us-west2
///       subnetwork: default
///       addressType: INTERNAL
///   pscIlbConsumer:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_consumer
///     properties:
///       name: psc-ilb-consumer-forwarding-rule
///       region: us-west2
///       target: ${pscIlbServiceAttachment.id}
///       loadBalancingScheme: ""
///       network: default
///       ipAddress: ${pscIlbConsumerAddress.id}
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
/// ### Service Attachment Explicit Networks
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       enableProxyProtocol: false
///       connectionPreference: ACCEPT_MANUAL
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///       consumerAcceptLists:
///         - networkUrl: ${pscIlbConsumerNetwork.selfLink}
///           connectionLimit: 1
///   pscIlbConsumerNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_consumer_network
///     properties:
///       name: psc-ilb-consumer-network
///       autoCreateSubnetworks: false
///   pscIlbConsumerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_consumer_subnetwork
///     properties:
///       name: psc-ilb-consumer-network
///       ipCidrRange: 10.0.0.0/16
///       region: us-west2
///       network: ${pscIlbConsumerNetwork.id}
///   pscIlbConsumerAddress:
///     type: gcp:compute:Address
///     name: psc_ilb_consumer_address
///     properties:
///       name: psc-ilb-consumer-address
///       region: us-west2
///       subnetwork: ${pscIlbConsumerSubnetwork.id}
///       addressType: INTERNAL
///   pscIlbConsumer:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_consumer
///     properties:
///       name: psc-ilb-consumer-forwarding-rule
///       region: us-west2
///       target: ${pscIlbServiceAttachment.id}
///       loadBalancingScheme: ""
///       network: ${pscIlbConsumerNetwork.id}
///       subnetwork: ${pscIlbConsumerSubnetwork.id}
///       ipAddress: ${pscIlbConsumerAddress.id}
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
/// ### Service Attachment Reconcile Connections
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       domainNames:
///         - gcp.tfacc.hashicorptest.com.
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_MANUAL
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///       consumerRejectLists:
///         - '673497134629'
///         - '482878270665'
///       consumerAcceptLists:
///         - projectIdOrNum: '658859330310'
///           connectionLimit: 4
///       reconcileConnections: false
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
///
/// ## Import
///
/// ServiceAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/serviceAttachments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ServiceAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default projects/{{project}}/regions/{{region}}/serviceAttachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceAttachmentArgs {
        /// The connection preference to use for this service attachment. Valid
        /// values include "ACCEPT_AUTOMATIC", "ACCEPT_MANUAL".
        #[builder(into)]
        pub connection_preference: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An array of projects that are allowed to connect to this service
        /// attachment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub consumer_accept_lists: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::ServiceAttachmentConsumerAcceptList>,
            >,
        >,
        /// An array of projects that are not allowed to connect to this service
        /// attachment.
        #[builder(into, default)]
        pub consumer_reject_lists: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If specified, the domain name will be used during the integration between
        /// the PSC connected endpoints and the Cloud DNS. For example, this is a
        /// valid domain name: "p.mycompany.com.". Current max number of domain names
        /// supported is 1.
        #[builder(into, default)]
        pub domain_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// If true, enable the proxy protocol which is for supplying client TCP/IP
        /// address data in TCP connections that traverse proxies on their way to
        /// destination servers.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub enable_proxy_protocol: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of subnets that is provided for NAT in this service attachment.
        #[builder(into)]
        pub nat_subnets: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.
        /// This limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.
        /// If the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.
        /// If the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.
        /// If unspecified, the default propagated connection limit is 250.
        #[builder(into, default)]
        pub propagated_connection_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// This flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.
        /// If false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .
        /// If true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list.
        #[builder(into, default)]
        pub reconcile_connections: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// URL of the region where the resource resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of a service serving the endpoint identified by this service attachment.
        #[builder(into)]
        pub target_service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceAttachmentResult {
        /// An array of the consumer forwarding rules connected to this service
        /// attachment.
        /// Structure is documented below.
        pub connected_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::ServiceAttachmentConnectedEndpoint>,
        >,
        /// The connection preference to use for this service attachment. Valid
        /// values include "ACCEPT_AUTOMATIC", "ACCEPT_MANUAL".
        pub connection_preference: pulumi_gestalt_rust::Output<String>,
        /// An array of projects that are allowed to connect to this service
        /// attachment.
        /// Structure is documented below.
        pub consumer_accept_lists: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::compute::ServiceAttachmentConsumerAcceptList>,
            >,
        >,
        /// An array of projects that are not allowed to connect to this service
        /// attachment.
        pub consumer_reject_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If specified, the domain name will be used during the integration between
        /// the PSC connected endpoints and the Cloud DNS. For example, this is a
        /// valid domain name: "p.mycompany.com.". Current max number of domain names
        /// supported is 1.
        pub domain_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// If true, enable the proxy protocol which is for supplying client TCP/IP
        /// address data in TCP connections that traverse proxies on their way to
        /// destination servers.
        ///
        ///
        /// - - -
        pub enable_proxy_protocol: pulumi_gestalt_rust::Output<bool>,
        /// Fingerprint of this resource. This field is used internally during
        /// updates of this resource.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An array of subnets that is provided for NAT in this service attachment.
        pub nat_subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.
        /// This limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.
        /// If the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.
        /// If the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.
        /// If unspecified, the default propagated connection limit is 250.
        pub propagated_connection_limit: pulumi_gestalt_rust::Output<i32>,
        /// This flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.
        /// If false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .
        /// If true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list.
        pub reconcile_connections: pulumi_gestalt_rust::Output<bool>,
        /// URL of the region where the resource resides.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The URL of a service serving the endpoint identified by this service attachment.
        pub target_service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceAttachmentArgs,
    ) -> ServiceAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_preference_binding = args
            .connection_preference
            .get_output(context);
        let consumer_accept_lists_binding = args
            .consumer_accept_lists
            .get_output(context);
        let consumer_reject_lists_binding = args
            .consumer_reject_lists
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let domain_names_binding = args.domain_names.get_output(context);
        let enable_proxy_protocol_binding = args
            .enable_proxy_protocol
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let nat_subnets_binding = args.nat_subnets.get_output(context);
        let project_binding = args.project.get_output(context);
        let propagated_connection_limit_binding = args
            .propagated_connection_limit
            .get_output(context);
        let reconcile_connections_binding = args
            .reconcile_connections
            .get_output(context);
        let region_binding = args.region.get_output(context);
        let target_service_binding = args.target_service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/serviceAttachment:ServiceAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionPreference".into(),
                    value: &connection_preference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerAcceptLists".into(),
                    value: &consumer_accept_lists_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerRejectLists".into(),
                    value: &consumer_reject_lists_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNames".into(),
                    value: &domain_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableProxyProtocol".into(),
                    value: &enable_proxy_protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "natSubnets".into(),
                    value: &nat_subnets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "propagatedConnectionLimit".into(),
                    value: &propagated_connection_limit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reconcileConnections".into(),
                    value: &reconcile_connections_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetService".into(),
                    value: &target_service_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceAttachmentResult {
            connected_endpoints: o.get_field("connectedEndpoints"),
            connection_preference: o.get_field("connectionPreference"),
            consumer_accept_lists: o.get_field("consumerAcceptLists"),
            consumer_reject_lists: o.get_field("consumerRejectLists"),
            description: o.get_field("description"),
            domain_names: o.get_field("domainNames"),
            enable_proxy_protocol: o.get_field("enableProxyProtocol"),
            fingerprint: o.get_field("fingerprint"),
            name: o.get_field("name"),
            nat_subnets: o.get_field("natSubnets"),
            project: o.get_field("project"),
            propagated_connection_limit: o.get_field("propagatedConnectionLimit"),
            reconcile_connections: o.get_field("reconcileConnections"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            target_service: o.get_field("targetService"),
        }
    }
}
