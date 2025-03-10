/// Manages a Firewall Rule associated with a Redis Cache.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         azi_id: 1
///       byteLength: 8
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: redis-resourcegroup
///       location: West Europe
///   exampleCache:
///     type: azure:redis:Cache
///     name: example
///     properties:
///       name: redis${server.hex}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   exampleFirewallRule:
///     type: azure:redis:FirewallRule
///     name: example
///     properties:
///       name: someIPrange
///       redisCacheName: ${exampleCache.name}
///       resourceGroupName: ${example.name}
///       startIp: 1.2.3.4
///       endIp: 2.3.4.5
/// ```
///
/// ## Import
///
/// Redis Firewall Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/firewallRule:FirewallRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1/firewallRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// The highest IP address included in the range.
        #[builder(into)]
        pub end_ip: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Redis Cache. Changing this forces a new resource to be created.
        #[builder(into)]
        pub redis_cache_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which this Redis Cache exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The lowest IP address included in the range
        #[builder(into)]
        pub start_ip: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// The highest IP address included in the range.
        pub end_ip: pulumi_gestalt_rust::Output<String>,
        /// The name of the Firewall Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Redis Cache. Changing this forces a new resource to be created.
        pub redis_cache_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which this Redis Cache exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The lowest IP address included in the range
        pub start_ip: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallRuleArgs,
    ) -> FirewallRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let end_ip_binding = args.end_ip.get_output(context);
        let name_binding = args.name.get_output(context);
        let redis_cache_name_binding = args.redis_cache_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let start_ip_binding = args.start_ip.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:redis/firewallRule:FirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endIp".into(),
                    value: &end_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisCacheName".into(),
                    value: &redis_cache_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startIp".into(),
                    value: &start_ip_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallRuleResult {
            end_ip: o.get_field("endIp"),
            name: o.get_field("name"),
            redis_cache_name: o.get_field("redisCacheName"),
            resource_group_name: o.get_field("resourceGroupName"),
            start_ip: o.get_field("startIp"),
        }
    }
}
