/// Manages a Redis Linked Server (ie Geo Location)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example-primary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources-primary
///       location: East US
///   example-primaryCache:
///     type: azure:redis:Cache
///     name: example-primary
///     properties:
///       name: example-cache1
///       location: ${["example-primary"].location}
///       resourceGroupName: ${["example-primary"].name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   example-secondary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources-secondary
///       location: West US
///   example-secondaryCache:
///     type: azure:redis:Cache
///     name: example-secondary
///     properties:
///       name: example-cache2
///       location: ${["example-secondary"].location}
///       resourceGroupName: ${["example-secondary"].name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   example-link:
///     type: azure:redis:LinkedServer
///     properties:
///       targetRedisCacheName: ${["example-primaryCache"].name}
///       resourceGroupName: ${["example-primaryCache"].resourceGroupName}
///       linkedRedisCacheId: ${["example-secondaryCache"].id}
///       linkedRedisCacheLocation: ${["example-secondaryCache"].location}
///       serverRole: Secondary
/// ```
///
/// ## Import
///
/// Redis can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/linkedServer:LinkedServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1/linkedServers/cache2
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServerArgs {
        /// The ID of the linked Redis cache. Changing this forces a new Redis to be created.
        #[builder(into)]
        pub linked_redis_cache_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the linked Redis cache. Changing this forces a new Redis to be created.
        #[builder(into)]
        pub linked_redis_cache_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Redis caches exists. Changing this forces a new Redis to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role of the linked Redis cache (eg "Secondary"). Changing this forces a new Redis to be created. Possible values are `Primary` and `Secondary`.
        #[builder(into)]
        pub server_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of Redis cache to link with. Changing this forces a new Redis to be created. (eg The primary role)
        #[builder(into)]
        pub target_redis_cache_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServerResult {
        /// The geo-replicated primary hostname for this linked server.
        pub geo_replicated_primary_host_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the linked Redis cache. Changing this forces a new Redis to be created.
        pub linked_redis_cache_id: pulumi_gestalt_rust::Output<String>,
        /// The location of the linked Redis cache. Changing this forces a new Redis to be created.
        pub linked_redis_cache_location: pulumi_gestalt_rust::Output<String>,
        /// The name of the linked server.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Redis caches exists. Changing this forces a new Redis to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The role of the linked Redis cache (eg "Secondary"). Changing this forces a new Redis to be created. Possible values are `Primary` and `Secondary`.
        pub server_role: pulumi_gestalt_rust::Output<String>,
        /// The name of Redis cache to link with. Changing this forces a new Redis to be created. (eg The primary role)
        pub target_redis_cache_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServerArgs,
    ) -> LinkedServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let linked_redis_cache_id_binding = args
            .linked_redis_cache_id
            .get_output(context);
        let linked_redis_cache_location_binding = args
            .linked_redis_cache_location
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let server_role_binding = args.server_role.get_output(context);
        let target_redis_cache_name_binding = args
            .target_redis_cache_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:redis/linkedServer:LinkedServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedRedisCacheId".into(),
                    value: &linked_redis_cache_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedRedisCacheLocation".into(),
                    value: &linked_redis_cache_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverRole".into(),
                    value: &server_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRedisCacheName".into(),
                    value: &target_redis_cache_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServerResult {
            geo_replicated_primary_host_name: o
                .get_field("geoReplicatedPrimaryHostName"),
            linked_redis_cache_id: o.get_field("linkedRedisCacheId"),
            linked_redis_cache_location: o.get_field("linkedRedisCacheLocation"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_role: o.get_field("serverRole"),
            target_redis_cache_name: o.get_field("targetRedisCacheName"),
        }
    }
}
