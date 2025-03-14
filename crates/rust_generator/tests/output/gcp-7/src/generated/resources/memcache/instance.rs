/// A Google Cloud Memcache instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/memorystore/docs/memcached/reference/rest/v1/projects.locations.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/memcache/docs/creating-instances)
///
/// ## Example Usage
///
/// ### Memcache Instance Basic
///
///
/// ```yaml
/// resources:
///   # This example assumes this network already exists.
///   # // The API creates a tenant network per network authorized for a
///   # // Memcache instance and that network is not deleted when the user-created
///   # // network (authorized_network) is deleted, so this prevents issues
///   # // with tenant network quota.
///   # // If this network hasn't been created and you are using this example in your
///   # // config, add an additional network resource or change
///   # // this from "data"to "resource"
///   memcacheNetwork:
///     type: gcp:compute:Network
///     name: memcache_network
///     properties:
///       name: test-network
///   serviceRange:
///     type: gcp:compute:GlobalAddress
///     name: service_range
///     properties:
///       name: address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${memcacheNetwork.id}
///   privateServiceConnection:
///     type: gcp:servicenetworking:Connection
///     name: private_service_connection
///     properties:
///       network: ${memcacheNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${serviceRange.name}
///   instance:
///     type: gcp:memcache:Instance
///     properties:
///       name: test-instance
///       authorizedNetwork: ${privateServiceConnection.network}
///       labels:
///         env: test
///       nodeConfig:
///         cpuCount: 1
///         memorySizeMb: 1024
///       nodeCount: 1
///       memcacheVersion: MEMCACHE_1_5
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: SATURDAY
///             duration: 14400s
///             startTime:
///               hours: 0
///               minutes: 30
///               seconds: 0
///               nanos: 0
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The full name of the GCE network to connect the instance to. If not provided, 'default' will be used.
        #[builder(into, default)]
        pub authorized_network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A user-visible name for the instance.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maintenance policy for an instance.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::memcache::InstanceMaintenancePolicy>,
        >,
        /// User-specified parameters for this memcache instance.
        #[builder(into, default)]
        pub memcache_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::memcache::InstanceMemcacheParameters>,
        >,
        /// The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest
        /// supported major version is MEMCACHE_1_5. The minor version will be automatically determined by our system based on the
        /// latest supported minor version. Default value: "MEMCACHE_1_5" Possible values: ["MEMCACHE_1_5", "MEMCACHE_1_6_15"]
        #[builder(into, default)]
        pub memcache_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the instance.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for memcache nodes.
        /// Structure is documented below.
        #[builder(into)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::memcache::InstanceNodeConfig,
        >,
        /// Number of nodes in the memcache instance.
        #[builder(into)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the Memcache instance. If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Contains the name of allocated IP address ranges associated with the private service access connection for example,
        /// "test-default" associated with IP range 10.0.0.0/29.
        #[builder(into, default)]
        pub reserved_ip_range_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Zones where memcache nodes should be provisioned. If not provided, all zones will be used.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// The full name of the GCE network to connect the instance to. If not provided, 'default' will be used.
        pub authorized_network: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Endpoint for Discovery API
        pub discovery_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A user-visible name for the instance.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maintenance policy for an instance.
        pub maintenance_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::memcache::InstanceMaintenancePolicy>,
        >,
        /// Output only. Published maintenance schedule.
        /// Structure is documented below.
        pub maintenance_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memcache::InstanceMaintenanceSchedule>,
        >,
        /// The full version of memcached server running on this instance.
        pub memcache_full_version: pulumi_gestalt_rust::Output<String>,
        /// Additional information about the instance state, if available.
        /// Structure is documented below.
        pub memcache_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memcache::InstanceMemcacheNode>,
        >,
        /// User-specified parameters for this memcache instance.
        pub memcache_parameters: pulumi_gestalt_rust::Output<
            Option<super::super::types::memcache::InstanceMemcacheParameters>,
        >,
        /// The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest
        /// supported major version is MEMCACHE_1_5. The minor version will be automatically determined by our system based on the
        /// latest supported minor version. Default value: "MEMCACHE_1_5" Possible values: ["MEMCACHE_1_5", "MEMCACHE_1_6_15"]
        pub memcache_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for memcache nodes.
        /// Structure is documented below.
        pub node_config: pulumi_gestalt_rust::Output<
            super::super::types::memcache::InstanceNodeConfig,
        >,
        /// Number of nodes in the memcache instance.
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the Memcache instance. If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Contains the name of allocated IP address ranges associated with the private service access connection for example,
        /// "test-default" associated with IP range 10.0.0.0/29.
        pub reserved_ip_range_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Zones where memcache nodes should be provisioned. If not provided, all zones will be used.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorized_network_binding = args.authorized_network.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let maintenance_policy_binding = args.maintenance_policy.get_output(context);
        let memcache_parameters_binding = args.memcache_parameters.get_output(context);
        let memcache_version_binding = args.memcache_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let node_count_binding = args.node_count.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let reserved_ip_range_ids_binding = args
            .reserved_ip_range_ids
            .get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:memcache/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedNetwork".into(),
                    value: &authorized_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenancePolicy".into(),
                    value: &maintenance_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memcacheParameters".into(),
                    value: &memcache_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memcacheVersion".into(),
                    value: &memcache_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservedIpRangeIds".into(),
                    value: &reserved_ip_range_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            authorized_network: o.get_field("authorizedNetwork"),
            create_time: o.get_field("createTime"),
            discovery_endpoint: o.get_field("discoveryEndpoint"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            maintenance_policy: o.get_field("maintenancePolicy"),
            maintenance_schedules: o.get_field("maintenanceSchedules"),
            memcache_full_version: o.get_field("memcacheFullVersion"),
            memcache_nodes: o.get_field("memcacheNodes"),
            memcache_parameters: o.get_field("memcacheParameters"),
            memcache_version: o.get_field("memcacheVersion"),
            name: o.get_field("name"),
            node_config: o.get_field("nodeConfig"),
            node_count: o.get_field("nodeCount"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            reserved_ip_range_ids: o.get_field("reservedIpRangeIds"),
            zones: o.get_field("zones"),
        }
    }
}
