/// Represents a TargetHttpProxy resource, which is used by one or more global
/// forwarding rule to route incoming HTTP requests to a URL map.
///
///
/// To get more information about TargetHttpProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetHttpProxies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/http/target-proxies)
///
/// ## Example Usage
///
/// ### Target Http Proxy Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_http_proxy::create(
///         "default",
///         TargetHttpProxyArgs::builder()
///             .name("test-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultBackendService = backend_service::create(
///         "defaultBackendService",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend-service")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("http-health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_service("${defaultBackendService.id}")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map")
///             .path_matchers(
///                 vec![
///                     UrlMapPathMatcher::builder()
///                     .defaultService("${defaultBackendService.id}").name("allpaths")
///                     .pathRules(vec![UrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/*",]).service("${defaultBackendService.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Target Http Proxy Http Keep Alive Timeout
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_http_proxy::create(
///         "default",
///         TargetHttpProxyArgs::builder()
///             .http_keep_alive_timeout_sec(610)
///             .name("test-http-keep-alive-timeout-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultBackendService = backend_service::create(
///         "defaultBackendService",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("backend-service")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("http-health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_service("${defaultBackendService.id}")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map")
///             .path_matchers(
///                 vec![
///                     UrlMapPathMatcher::builder()
///                     .defaultService("${defaultBackendService.id}").name("allpaths")
///                     .pathRules(vec![UrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/*",]).service("${defaultBackendService.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Target Http Proxy Https Redirect
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_http_proxy::create(
///         "default",
///         TargetHttpProxyArgs::builder()
///             .name("test-https-redirect-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_url_redirect(
///                 UrlMapDefaultUrlRedirect::builder()
///                     .httpsRedirect(true)
///                     .stripQuery(false)
///                     .build_struct(),
///             )
///             .name("url-map")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TargetHttpProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetHttpProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetHttpProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpProxy:TargetHttpProxy default projects/{{project}}/global/targetHttpProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpProxy:TargetHttpProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpProxy:TargetHttpProxy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_http_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetHttpProxyArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value will be used. For Global
        /// external HTTP(S) load balancer, the default value is 610 seconds, the
        /// minimum allowed value is 5 seconds and the maximum allowed value is 1200
        /// seconds. For cross-region internal HTTP(S) load balancer, the default
        /// value is 600 seconds, the minimum allowed value is 5 seconds, and the
        /// maximum allowed value is 600 seconds. For Global external HTTP(S) load
        /// balancer (classic), this option is not available publicly.
        #[builder(into, default)]
        pub http_keep_alive_timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
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
        /// A reference to the UrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub url_map: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetHttpProxyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value will be used. For Global
        /// external HTTP(S) load balancer, the default value is 610 seconds, the
        /// minimum allowed value is 5 seconds and the maximum allowed value is 1200
        /// seconds. For cross-region internal HTTP(S) load balancer, the default
        /// value is 600 seconds, the minimum allowed value is 5 seconds, and the
        /// maximum allowed value is 600 seconds. For Global external HTTP(S) load
        /// balancer (classic), this option is not available publicly.
        pub http_keep_alive_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
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
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_gestalt_rust::Output<i32>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A reference to the UrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        pub url_map: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetHttpProxyArgs,
    ) -> TargetHttpProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let http_keep_alive_timeout_sec_binding = args
            .http_keep_alive_timeout_sec
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let proxy_bind_binding = args.proxy_bind.get_output(context);
        let url_map_binding = args.url_map.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/targetHttpProxy:TargetHttpProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpKeepAliveTimeoutSec".into(),
                    value: &http_keep_alive_timeout_sec_binding.drop_type(),
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
                    name: "urlMap".into(),
                    value: &url_map_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetHttpProxyResult {
            id: o.get_field("id"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            http_keep_alive_timeout_sec: o.get_field("httpKeepAliveTimeoutSec"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            proxy_bind: o.get_field("proxyBind"),
            proxy_id: o.get_field("proxyId"),
            self_link: o.get_field("selfLink"),
            url_map: o.get_field("urlMap"),
        }
    }
}
