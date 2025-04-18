/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for Elastic.
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
///     let exampleSpringCloudElasticApplicationPerformanceMonitoring = spring_cloud_elastic_application_performance_monitoring::create(
///         "exampleSpringCloudElasticApplicationPerformanceMonitoring",
///         SpringCloudElasticApplicationPerformanceMonitoringArgs::builder()
///             .application_packages(vec!["org.example", "org.another.example",])
///             .globally_enabled(true)
///             .name("example")
///             .server_url("http://127.0.0.1:8200")
///             .service_name("example-service-name")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
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
/// Spring Cloud Application Performance Monitoring resource for Elastic can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudElasticApplicationPerformanceMonitoring:SpringCloudElasticApplicationPerformanceMonitoring example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AppPlatform/spring/service1/apms/apm1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_elastic_application_performance_monitoring {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudElasticApplicationPerformanceMonitoringArgs {
        /// Specifies a list of the packages which should be used to determine whether a stack trace frame is an in-app frame or a library frame. This is a comma separated list of package names.
        #[builder(into)]
        pub application_packages: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Elastic. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the server URL. The URL must be fully qualified, including protocol (http or https) and port.
        #[builder(into)]
        pub server_url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the service name which is used to keep all the errors and transactions of your service together and is the primary filter in the Elastic APM user interface.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudElasticApplicationPerformanceMonitoringResult {
        /// Specifies a list of the packages which should be used to determine whether a stack trace frame is an in-app frame or a library frame. This is a comma separated list of package names.
        pub application_packages: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Elastic. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the server URL. The URL must be fully qualified, including protocol (http or https) and port.
        pub server_url: pulumi_gestalt_rust::Output<String>,
        /// Specifies the service name which is used to keep all the errors and transactions of your service together and is the primary filter in the Elastic APM user interface.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudElasticApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudElasticApplicationPerformanceMonitoringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_packages_binding = args.application_packages.get_output(context);
        let globally_enabled_binding = args.globally_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let server_url_binding = args.server_url.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudElasticApplicationPerformanceMonitoring:SpringCloudElasticApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationPackages".into(),
                    value: &application_packages_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globallyEnabled".into(),
                    value: &globally_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverUrl".into(),
                    value: &server_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudElasticApplicationPerformanceMonitoringResult {
            application_packages: o.get_field("applicationPackages"),
            globally_enabled: o.get_field("globallyEnabled"),
            name: o.get_field("name"),
            server_url: o.get_field("serverUrl"),
            service_name: o.get_field("serviceName"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
        }
    }
}
