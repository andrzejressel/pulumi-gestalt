/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
///
/// Manages a Spring Cloud Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleSpringCloudGateway = spring_cloud_gateway::create(
///         "exampleSpringCloudGateway",
///         SpringCloudGatewayArgs::builder()
///             .api_metadata(
///                 SpringCloudGatewayApiMetadata::builder()
///                     .description("example description")
///                     .documentationUrl("https://www.example.com/docs")
///                     .serverUrl("https://wwww.example.com")
///                     .title("example title")
///                     .version("1.0")
///                     .build_struct(),
///             )
///             .cors(
///                 SpringCloudGatewayCors::builder()
///                     .allowedHeaders(vec!["*",])
///                     .allowedMethods(vec!["PUT",])
///                     .allowedOrigins(vec!["example.com",])
///                     .credentialsAllowed(false)
///                     .exposedHeaders(vec!["x-example-header",])
///                     .maxAgeSeconds(86400)
///                     .build_struct(),
///             )
///             .https_only(false)
///             .instance_count(2)
///             .local_response_cache_per_instance(
///                 SpringCloudGatewayLocalResponseCachePerInstance::builder()
///                     .size("100MB")
///                     .timeToLive("30s")
///                     .build_struct(),
///             )
///             .name("default")
///             .public_network_access_enabled(true)
///             .quota(
///                 SpringCloudGatewayQuota::builder().cpu("1").memory("2Gi").build_struct(),
///             )
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .sso(
///                 SpringCloudGatewaySso::builder()
///                     .clientId("example id")
///                     .clientSecret("example secret")
///                     .issuerUri("https://www.test.com/issueToken")
///                     .scopes(vec!["read",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudGateway:SpringCloudGateway example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/gateways/gateway1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudGatewayArgs {
        /// A `api_metadata` block as defined below.
        #[builder(into, default)]
        pub api_metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudGatewayApiMetadata>,
        >,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        #[builder(into, default)]
        pub application_performance_monitoring_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies a list of application performance monitoring types used in the Spring Cloud Gateway. The allowed values are `AppDynamics`, `ApplicationInsights`, `Dynatrace`, `ElasticAPM` and `NewRelic`.
        #[builder(into, default)]
        pub application_performance_monitoring_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A `client_authorization` block as defined below.
        #[builder(into, default)]
        pub client_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudGatewayClientAuthorization,
            >,
        >,
        /// A `cors` block as defined below.
        #[builder(into, default)]
        pub cors: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudGatewayCors>,
        >,
        /// Specifies the environment variables of the Spring Cloud Gateway as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// is only https is allowed?
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the required instance count of the Spring Cloud Gateway. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A `local_response_cache_per_instance` block as defined below. Only one of `local_response_cache_per_instance` or `local_response_cache_per_route` can be specified.
        #[builder(into, default)]
        pub local_response_cache_per_instance: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudGatewayLocalResponseCachePerInstance,
            >,
        >,
        /// A `local_response_cache_per_route` block as defined below. Only one of `local_response_cache_per_instance` or `local_response_cache_per_route` can be specified.
        #[builder(into, default)]
        pub local_response_cache_per_route: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudGatewayLocalResponseCachePerRoute,
            >,
        >,
        /// The name which should be used for this Spring Cloud Gateway. Changing this forces a new Spring Cloud Gateway to be created. The only possible value is `default`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the Spring Cloud Gateway exposes endpoint.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudGatewayQuota>,
        >,
        /// Specifies the sensitive environment variables of the Spring Cloud Gateway as a map of key-value pairs.
        #[builder(into, default)]
        pub sensitive_environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Gateway to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sso` block as defined below.
        #[builder(into, default)]
        pub sso: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudGatewaySso>,
        >,
    }
    #[allow(dead_code)]
    pub struct SpringCloudGatewayResult {
        /// A `api_metadata` block as defined below.
        pub api_metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudGatewayApiMetadata>,
        >,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        pub application_performance_monitoring_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Specifies a list of application performance monitoring types used in the Spring Cloud Gateway. The allowed values are `AppDynamics`, `ApplicationInsights`, `Dynatrace`, `ElasticAPM` and `NewRelic`.
        pub application_performance_monitoring_types: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// A `client_authorization` block as defined below.
        pub client_authorization: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudGatewayClientAuthorization,
            >,
        >,
        /// A `cors` block as defined below.
        pub cors: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudGatewayCors>,
        >,
        /// Specifies the environment variables of the Spring Cloud Gateway as a map of key-value pairs.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// is only https is allowed?
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the required instance count of the Spring Cloud Gateway. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A `local_response_cache_per_instance` block as defined below. Only one of `local_response_cache_per_instance` or `local_response_cache_per_route` can be specified.
        pub local_response_cache_per_instance: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudGatewayLocalResponseCachePerInstance,
            >,
        >,
        /// A `local_response_cache_per_route` block as defined below. Only one of `local_response_cache_per_instance` or `local_response_cache_per_route` can be specified.
        pub local_response_cache_per_route: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudGatewayLocalResponseCachePerRoute,
            >,
        >,
        /// The name which should be used for this Spring Cloud Gateway. Changing this forces a new Spring Cloud Gateway to be created. The only possible value is `default`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the Spring Cloud Gateway exposes endpoint.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `quota` block as defined below.
        pub quota: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudGatewayQuota,
        >,
        /// Specifies the sensitive environment variables of the Spring Cloud Gateway as a map of key-value pairs.
        pub sensitive_environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Gateway to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
        /// A `sso` block as defined below.
        pub sso: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudGatewaySso>,
        >,
        /// URL of the Spring Cloud Gateway, exposed when 'public_network_access_enabled' is true.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudGatewayArgs,
    ) -> SpringCloudGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_metadata_binding = args.api_metadata.get_output(context);
        let application_performance_monitoring_ids_binding = args
            .application_performance_monitoring_ids
            .get_output(context);
        let application_performance_monitoring_types_binding = args
            .application_performance_monitoring_types
            .get_output(context);
        let client_authorization_binding = args.client_authorization.get_output(context);
        let cors_binding = args.cors.get_output(context);
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let https_only_binding = args.https_only.get_output(context);
        let instance_count_binding = args.instance_count.get_output(context);
        let local_response_cache_per_instance_binding = args
            .local_response_cache_per_instance
            .get_output(context);
        let local_response_cache_per_route_binding = args
            .local_response_cache_per_route
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let quota_binding = args.quota.get_output(context);
        let sensitive_environment_variables_binding = args
            .sensitive_environment_variables
            .get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let sso_binding = args.sso.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudGateway:SpringCloudGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiMetadata".into(),
                    value: &api_metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationPerformanceMonitoringIds".into(),
                    value: &application_performance_monitoring_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationPerformanceMonitoringTypes".into(),
                    value: &application_performance_monitoring_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientAuthorization".into(),
                    value: &client_authorization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cors".into(),
                    value: &cors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localResponseCachePerInstance".into(),
                    value: &local_response_cache_per_instance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localResponseCachePerRoute".into(),
                    value: &local_response_cache_per_route_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quota".into(),
                    value: &quota_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sensitiveEnvironmentVariables".into(),
                    value: &sensitive_environment_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sso".into(),
                    value: &sso_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudGatewayResult {
            api_metadata: o.get_field("apiMetadata"),
            application_performance_monitoring_ids: o
                .get_field("applicationPerformanceMonitoringIds"),
            application_performance_monitoring_types: o
                .get_field("applicationPerformanceMonitoringTypes"),
            client_authorization: o.get_field("clientAuthorization"),
            cors: o.get_field("cors"),
            environment_variables: o.get_field("environmentVariables"),
            https_only: o.get_field("httpsOnly"),
            instance_count: o.get_field("instanceCount"),
            local_response_cache_per_instance: o
                .get_field("localResponseCachePerInstance"),
            local_response_cache_per_route: o.get_field("localResponseCachePerRoute"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            quota: o.get_field("quota"),
            sensitive_environment_variables: o
                .get_field("sensitiveEnvironmentVariables"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
            sso: o.get_field("sso"),
            url: o.get_field("url"),
        }
    }
}
