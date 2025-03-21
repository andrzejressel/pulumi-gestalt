/// Represents a RegionTargetTcpProxy resource, which is used by one or more
/// forwarding rules to route incoming TCP requests to a regional TCP proxy load
/// balancer.
///
///
/// To get more information about RegionTargetTcpProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionTargetTcpProxies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/tcp/internal-proxy)
///
/// ## Example Usage
///
/// ### Region Target Tcp Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionTargetTcpProxy
///     properties:
///       name: test-proxy
///       region: europe-west4
///       backendService: ${defaultRegionBackendService.id}
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       name: backend-service
///       protocol: TCP
///       timeoutSec: 10
///       region: europe-west4
///       healthChecks: ${defaultRegionHealthCheck.id}
///       loadBalancingScheme: INTERNAL_MANAGED
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       name: health-check
///       region: europe-west4
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '80'
/// ```
///
/// ## Import
///
/// RegionTargetTcpProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/targetTcpProxies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionTargetTcpProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetTcpProxy:RegionTargetTcpProxy default projects/{{project}}/regions/{{region}}/targetTcpProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetTcpProxy:RegionTargetTcpProxy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetTcpProxy:RegionTargetTcpProxy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetTcpProxy:RegionTargetTcpProxy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_target_tcp_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionTargetTcpProxyArgs {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        #[builder(into, default)]
        pub proxy_bind: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        #[builder(into, default)]
        pub proxy_header: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created target TCP proxy should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionTargetTcpProxyResult {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        pub backend_service: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        pub proxy_bind: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        pub proxy_header: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_gestalt_rust::Output<i32>,
        /// The Region in which the created target TCP proxy should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionTargetTcpProxyArgs,
    ) -> RegionTargetTcpProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backend_service_binding = args.backend_service.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let proxy_bind_binding = args.proxy_bind.get_output(context);
        let proxy_header_binding = args.proxy_header.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionTargetTcpProxy:RegionTargetTcpProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendService".into(),
                    value: &backend_service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
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
                    name: "proxyBind".into(),
                    value: &proxy_bind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxyHeader".into(),
                    value: &proxy_header_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionTargetTcpProxyResult {
            backend_service: o.get_field("backendService"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            proxy_bind: o.get_field("proxyBind"),
            proxy_header: o.get_field("proxyHeader"),
            proxy_id: o.get_field("proxyId"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
        }
    }
}
