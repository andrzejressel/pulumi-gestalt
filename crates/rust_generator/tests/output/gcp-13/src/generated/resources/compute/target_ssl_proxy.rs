/// Represents a TargetSslProxy resource, which is used by one or more
/// global forwarding rule to route incoming SSL requests to a backend
/// service.
///
///
/// To get more information about TargetSslProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetSslProxies)
/// * How-to Guides
///     * [Setting Up SSL proxy for Google Cloud Load Balancing](https://cloud.google.com/compute/docs/load-balancing/tcp-ssl/)
///
/// ## Example Usage
///
/// ### Target Ssl Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetSSLProxy
///     properties:
///       name: test-proxy
///       backendService: ${defaultBackendService.id}
///       sslCertificates:
///         - ${defaultSSLCertificate.id}
///   defaultSSLCertificate:
///     type: gcp:compute:SSLCertificate
///     name: default
///     properties:
///       name: default-cert
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/private.key
///           return: result
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/certificate.crt
///           return: result
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       protocol: SSL
///       healthChecks: ${defaultHealthCheck.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '443'
/// ```
///
/// ## Import
///
/// TargetSslProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetSslProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetSslProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetSSLProxy:TargetSSLProxy default projects/{{project}}/global/targetSslProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetSSLProxy:TargetSSLProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetSSLProxy:TargetSSLProxy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_ssl_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetSSLProxyArgs {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A reference to the CertificateMap resource uri that identifies a certificate map
        /// associated with the given target proxy. This field can only be set for global target proxies.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}`.
        #[builder(into, default)]
        pub certificate_map: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        #[builder(into, default)]
        pub proxy_header: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of SslCertificate resources that are used to authenticate
        /// connections between users and the load balancer. At least one
        /// SSL certificate must be specified.
        #[builder(into, default)]
        pub ssl_certificates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A reference to the SslPolicy resource that will be associated with
        /// the TargetSslProxy resource. If not set, the TargetSslProxy
        /// resource will not have any SSL policy configured.
        #[builder(into, default)]
        pub ssl_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetSSLProxyResult {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        pub backend_service: pulumi_gestalt_rust::Output<String>,
        /// A reference to the CertificateMap resource uri that identifies a certificate map
        /// associated with the given target proxy. This field can only be set for global target proxies.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}`.
        pub certificate_map: pulumi_gestalt_rust::Output<Option<String>>,
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
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        pub proxy_header: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_gestalt_rust::Output<i32>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A list of SslCertificate resources that are used to authenticate
        /// connections between users and the load balancer. At least one
        /// SSL certificate must be specified.
        pub ssl_certificates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A reference to the SslPolicy resource that will be associated with
        /// the TargetSslProxy resource. If not set, the TargetSslProxy
        /// resource will not have any SSL policy configured.
        pub ssl_policy: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetSSLProxyArgs,
    ) -> TargetSSLProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backend_service_binding = args.backend_service.get_output(context);
        let certificate_map_binding = args.certificate_map.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let proxy_header_binding = args.proxy_header.get_output(context);
        let ssl_certificates_binding = args.ssl_certificates.get_output(context);
        let ssl_policy_binding = args.ssl_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/targetSSLProxy:TargetSSLProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendService".into(),
                    value: &backend_service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateMap".into(),
                    value: &certificate_map_binding.drop_type(),
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
                    name: "proxyHeader".into(),
                    value: &proxy_header_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslCertificates".into(),
                    value: &ssl_certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetSSLProxyResult {
            backend_service: o.get_field("backendService"),
            certificate_map: o.get_field("certificateMap"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            proxy_header: o.get_field("proxyHeader"),
            proxy_id: o.get_field("proxyId"),
            self_link: o.get_field("selfLink"),
            ssl_certificates: o.get_field("sslCertificates"),
            ssl_policy: o.get_field("sslPolicy"),
        }
    }
}
