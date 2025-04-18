/// Associates a Spring Cloud Application with a Redis Cache.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///   exampleCache:
///     type: azure:redis:Cache
///     name: example
///     properties:
///       name: example-cache
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       capacity: 0
///       family: C
///       skuName: Basic
///       enableNonSslPort: true
///   exampleSpringCloudAppRedisAssociation:
///     type: azure:appplatform:SpringCloudAppRedisAssociation
///     name: example
///     properties:
///       name: example-bind
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       redisCacheId: ${exampleCache.id}
///       redisAccessKey: ${exampleCache.primaryAccessKey}
///       sslEnabled: true
/// ```
///
/// ## Import
///
/// Spring Cloud Application Redis Association can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudAppRedisAssociation:SpringCloudAppRedisAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.AppPlatform/spring/myservice/apps/myapp/bindings/bind1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_app_redis_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppRedisAssociationArgs {
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Redis Cache access key.
        #[builder(into)]
        pub redis_access_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Redis Cache resource ID. Changing this forces a new resource to be created.
        #[builder(into)]
        pub redis_cache_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Spring Cloud Application resource ID in which the Association is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should SSL be used when connecting to Redis? Defaults to `true`.
        #[builder(into, default)]
        pub ssl_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppRedisAssociationResult {
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Redis Cache access key.
        pub redis_access_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Redis Cache resource ID. Changing this forces a new resource to be created.
        pub redis_cache_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Spring Cloud Application resource ID in which the Association is created. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
        /// Should SSL be used when connecting to Redis? Defaults to `true`.
        pub ssl_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudAppRedisAssociationArgs,
    ) -> SpringCloudAppRedisAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let redis_access_key_binding = args.redis_access_key.get_output(context);
        let redis_cache_id_binding = args.redis_cache_id.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let ssl_enabled_binding = args.ssl_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudAppRedisAssociation:SpringCloudAppRedisAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisAccessKey".into(),
                    value: &redis_access_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisCacheId".into(),
                    value: &redis_cache_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslEnabled".into(),
                    value: &ssl_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudAppRedisAssociationResult {
            name: o.get_field("name"),
            redis_access_key: o.get_field("redisAccessKey"),
            redis_cache_id: o.get_field("redisCacheId"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
            ssl_enabled: o.get_field("sslEnabled"),
        }
    }
}
