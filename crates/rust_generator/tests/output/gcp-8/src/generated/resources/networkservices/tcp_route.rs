/// ## Example Usage
///
/// ### Network Services Tcp Route Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultTcpRoute:
///     type: gcp:networkservices:TcpRoute
///     name: default
///     properties:
///       name: my-tcp-route
///       labels:
///         foo: bar
///       description: my description
///       rules:
///         - matches:
///             - address: 10.0.0.1/32
///               port: '8081'
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
///             originalDestination: false
/// ```
/// ### Network Services Tcp Route Actions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultTcpRoute:
///     type: gcp:networkservices:TcpRoute
///     name: default
///     properties:
///       name: my-tcp-route
///       labels:
///         foo: bar
///       description: my description
///       rules:
///         - action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
///             originalDestination: false
///             idleTimeout: 60s
/// ```
/// ### Network Services Tcp Route Mesh Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultMesh:
///     type: gcp:networkservices:Mesh
///     name: default
///     properties:
///       name: my-tcp-route
///       labels:
///         foo: bar
///       description: my description
///   defaultTcpRoute:
///     type: gcp:networkservices:TcpRoute
///     name: default
///     properties:
///       name: my-tcp-route
///       labels:
///         foo: bar
///       description: my description
///       meshes:
///         - ${defaultMesh.id}
///       rules:
///         - matches:
///             - address: 10.0.0.1/32
///               port: '8081'
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
///             originalDestination: false
/// ```
/// ### Network Services Tcp Route Gateway Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultGateway:
///     type: gcp:networkservices:Gateway
///     name: default
///     properties:
///       name: my-tcp-route
///       labels:
///         foo: bar
///       description: my description
///       scope: my-scope
///       type: OPEN_MESH
///       ports:
///         - 443
///   defaultTcpRoute:
///     type: gcp:networkservices:TcpRoute
///     name: default
///     properties:
///       name: my-tcp-route
///       labels:
///         foo: bar
///       description: my description
///       gateways:
///         - ${defaultGateway.id}
///       rules:
///         - matches:
///             - address: 10.0.0.1/32
///               port: '8081'
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
///             originalDestination: false
/// ```
///
/// ## Import
///
/// TcpRoute can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/tcpRoutes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TcpRoute can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/tcpRoute:TcpRoute default projects/{{project}}/locations/global/tcpRoutes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/tcpRoute:TcpRoute default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/tcpRoute:TcpRoute default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tcp_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TcpRouteArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        #[builder(into, default)]
        pub gateways: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of label tags associated with the TcpRoute resource. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name> The attached
        /// Mesh should be of a type SIDECAR
        #[builder(into, default)]
        pub meshes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the TcpRoute resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rules that define how traffic is routed and handled. At least one RouteRule must be supplied.
        /// If there are multiple rules then the action taken will be the first rule to match.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::networkservices::TcpRouteRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct TcpRouteResult {
        /// Time the TcpRoute was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        pub gateways: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Set of label tags associated with the TcpRoute resource. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name> The attached
        /// Mesh should be of a type SIDECAR
        pub meshes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the TcpRoute resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Rules that define how traffic is routed and handled. At least one RouteRule must be supplied.
        /// If there are multiple rules then the action taken will be the first rule to match.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkservices::TcpRouteRule>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Time the TcpRoute was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TcpRouteArgs,
    ) -> TcpRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let gateways_binding = args.gateways.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let meshes_binding = args.meshes.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkservices/tcpRoute:TcpRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gateways".into(),
                    value: &gateways_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshes".into(),
                    value: &meshes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TcpRouteResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            gateways: o.get_field("gateways"),
            labels: o.get_field("labels"),
            meshes: o.get_field("meshes"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rules: o.get_field("rules"),
            self_link: o.get_field("selfLink"),
            update_time: o.get_field("updateTime"),
        }
    }
}
