/// The Google Compute Engine Instance Group Manager API creates and manages pools
/// of homogeneous Compute Engine virtual machine instances from a common instance
/// template. For more information, see [the official documentation](https://cloud.google.com/compute/docs/instance-groups/manager)
/// and [API](https://cloud.google.com/compute/docs/reference/latest/instanceGroupManagers)
///
/// > **Note:** Use [gcp.compute.RegionInstanceGroupManager](https://www.terraform.io/docs/providers/google/r/compute_region_instance_group_manager.html) to create a regional (multi-zone) instance group manager.
///
/// ## Example Usage
///
/// ### With Top Level Instance Template (`Google` Provider)
///
/// ```yaml
/// resources:
///   autohealing:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: autohealing-health-check
///       checkIntervalSec: 5
///       timeoutSec: 5
///       healthyThreshold: 2
///       unhealthyThreshold: 10 # 50 seconds
///       httpHealthCheck:
///         requestPath: /healthz
///         port: '8080'
///   appserver:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       name: appserver-igm
///       baseInstanceName: app
///       zone: us-central1-a
///       versions:
///         - instanceTemplate: ${appserverGoogleComputeInstanceTemplate.selfLinkUnique}
///       allInstancesConfig:
///         metadata:
///           metadata_key: metadata_value
///         labels:
///           label_key: label_value
///       targetPools:
///         - ${appserverGoogleComputeTargetPool.id}
///       targetSize: 2
///       namedPorts:
///         - name: customhttp
///           port: 8888
///       autoHealingPolicies:
///         healthCheck: ${autohealing.id}
///         initialDelaySec: 300
/// ```
///
///
/// ### With Multiple Versions (`Google-Beta` Provider)
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appserver = instance_group_manager::create(
///         "appserver",
///         InstanceGroupManagerArgs::builder()
///             .base_instance_name("app")
///             .name("appserver-igm")
///             .target_size(5)
///             .versions(
///                 vec![
///                     InstanceGroupManagerVersion::builder()
///                     .instanceTemplate("${appserverGoogleComputeInstanceTemplate.selfLinkUnique}")
///                     .name("appserver").build_struct(),
///                     InstanceGroupManagerVersion::builder()
///                     .instanceTemplate("${[\"appserver-canary\"].selfLinkUnique}")
///                     .name("appserver-canary")
///                     .targetSize(InstanceGroupManagerVersionTargetSize::builder().fixed(1)
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Standby Policy (`Google-Beta` Provider)
/// ```yaml
/// resources:
///   igm-sr:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       name: tf-sr-igm
///       baseInstanceName: tf-sr-igm-instance
///       zone: us-central1-a
///       targetSize: 5
///       versions:
///         - instanceTemplate: ${["sr-igm"].selfLink}
///           name: primary
///       standbyPolicy:
///         initialDelaySec: 30
///         mode: MANUAL
///       targetSuspendedSize: 2
///       targetStoppedSize: 1
/// ```
///
/// ## Import
///
/// Instance group managers can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, instance group managers can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_group_manager {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupManagerArgs {
        /// Properties to set on all instances in the group. After setting
        /// allInstancesConfig on the group, you must update the group's instances to
        /// apply the configuration.
        #[builder(into, default)]
        pub all_instances_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceGroupManagerAllInstancesConfig>,
        >,
        /// The autohealing policies for this managed instance
        /// group. You can specify only one value. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances#monitoring_groups).
        #[builder(into, default)]
        pub auto_healing_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceGroupManagerAutoHealingPolicies>,
        >,
        /// The base instance name to use for
        /// instances in this group. The value must be a valid
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt) name. Supported characters
        /// are lowercase letters, numbers, and hyphens (-). Instances are named by
        /// appending a hyphen and a random four-character string to the base instance
        /// name.
        #[builder(into)]
        pub base_instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional textual description of the instance
        /// group manager.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The instance lifecycle policy for this managed instance group.
        #[builder(into, default)]
        pub instance_lifecycle_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceGroupManagerInstanceLifecyclePolicy,
            >,
        >,
        /// Pagination behavior of the `listManagedInstances` API
        /// method for this managed instance group. Valid values are: `PAGELESS`, `PAGINATED`.
        /// If `PAGELESS` (default), Pagination is disabled for the group's `listManagedInstances` API method.
        /// `maxResults` and `pageToken` query parameters are ignored and all instances are returned in a single
        /// response. If `PAGINATED`, pagination is enabled, `maxResults` and `pageToken` query parameters are
        /// respected.
        #[builder(into, default)]
        pub list_managed_instances_results: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the instance group manager. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The named port configuration. See the section below
        /// for details on configuration.
        #[builder(into, default)]
        pub named_ports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceGroupManagerNamedPort>>,
        >,
        /// Input only additional params for instance group manager creation. Structure is documented below. For more information, see [API](https://cloud.google.com/compute/docs/reference/rest/beta/instanceGroupManagers/insert).
        ///
        /// - - -
        #[builder(into, default)]
        pub params: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceGroupManagerParams>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The standby policy for stopped and suspended instances. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/suspended-and-stopped-vms-in-mig) and [API](https://cloud.google.com/compute/docs/reference/rest/beta/regionInstanceGroupManagers/patch)
        #[builder(into, default)]
        pub standby_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceGroupManagerStandbyPolicy>,
        >,
        /// Disks created on the instances that will be preserved on instance delete, update, etc. Structure is documented below. For more information see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/configuring-stateful-disks-in-migs).
        #[builder(into, default)]
        pub stateful_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceGroupManagerStatefulDisk>>,
        >,
        /// External network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        #[builder(into, default)]
        pub stateful_external_ips: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulExternalIp>,
            >,
        >,
        /// Internal network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        #[builder(into, default)]
        pub stateful_internal_ips: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulInternalIp>,
            >,
        >,
        /// The full URL of all target pools to which new
        /// instances in the group are added. Updating the target pools attribute does
        /// not affect existing instances.
        #[builder(into, default)]
        pub target_pools: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The target number of running instances for this managed instance group. This value should always be explicitly set
        /// unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0.
        #[builder(into, default)]
        pub target_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The target number of stopped instances for this managed instance group.
        #[builder(into, default)]
        pub target_stopped_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The target number of suspended instances for this managed instance group.
        #[builder(into, default)]
        pub target_suspended_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The update policy for this managed instance group. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/updating-managed-instance-groups) and [API](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagers/patch).
        #[builder(into, default)]
        pub update_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceGroupManagerUpdatePolicy>,
        >,
        /// Application versions managed by this instance group. Each
        /// version deals with a specific instance template, allowing canary release scenarios.
        /// Structure is documented below.
        #[builder(into)]
        pub versions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::InstanceGroupManagerVersion>,
        >,
        /// Whether to wait for all instances to be created/updated before
        /// returning. Note that if this is set to true and the operation does not succeed, this provider will
        /// continue trying until it times out.
        #[builder(into, default)]
        pub wait_for_instances: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When used with `wait_for_instances` it specifies the status to wait for.
        /// When `STABLE` is specified this resource will wait until the instances are stable before returning. When `UPDATED` is
        /// set, it will wait for the version target to be reached and any per instance configs to be effective as well as all
        /// instances to be stable before returning. The possible values are `STABLE` and `UPDATED`
        #[builder(into, default)]
        pub wait_for_instances_status: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The zone that instances in this group should be created
        /// in.
        ///
        /// - - -
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupManagerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Properties to set on all instances in the group. After setting
        /// allInstancesConfig on the group, you must update the group's instances to
        /// apply the configuration.
        pub all_instances_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerAllInstancesConfig>,
        >,
        /// The autohealing policies for this managed instance
        /// group. You can specify only one value. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances#monitoring_groups).
        pub auto_healing_policies: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerAutoHealingPolicies>,
        >,
        /// The base instance name to use for
        /// instances in this group. The value must be a valid
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt) name. Supported characters
        /// are lowercase letters, numbers, and hyphens (-). Instances are named by
        /// appending a hyphen and a random four-character string to the base instance
        /// name.
        pub base_instance_name: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional textual description of the instance
        /// group manager.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fingerprint of the instance group manager.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The full URL of the instance group created by the manager.
        pub instance_group: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub instance_group_manager_id: pulumi_gestalt_rust::Output<i32>,
        /// The instance lifecycle policy for this managed instance group.
        pub instance_lifecycle_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceGroupManagerInstanceLifecyclePolicy,
        >,
        /// Pagination behavior of the `listManagedInstances` API
        /// method for this managed instance group. Valid values are: `PAGELESS`, `PAGINATED`.
        /// If `PAGELESS` (default), Pagination is disabled for the group's `listManagedInstances` API method.
        /// `maxResults` and `pageToken` query parameters are ignored and all instances are returned in a single
        /// response. If `PAGINATED`, pagination is enabled, `maxResults` and `pageToken` query parameters are
        /// respected.
        pub list_managed_instances_results: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the instance group manager. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The named port configuration. See the section below
        /// for details on configuration.
        pub named_ports: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupManagerNamedPort>>,
        >,
        pub operation: pulumi_gestalt_rust::Output<String>,
        /// Input only additional params for instance group manager creation. Structure is documented below. For more information, see [API](https://cloud.google.com/compute/docs/reference/rest/beta/instanceGroupManagers/insert).
        ///
        /// - - -
        pub params: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerParams>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URL of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The standby policy for stopped and suspended instances. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/suspended-and-stopped-vms-in-mig) and [API](https://cloud.google.com/compute/docs/reference/rest/beta/regionInstanceGroupManagers/patch)
        pub standby_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceGroupManagerStandbyPolicy,
        >,
        /// Disks created on the instances that will be preserved on instance delete, update, etc. Structure is documented below. For more information see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/configuring-stateful-disks-in-migs).
        pub stateful_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupManagerStatefulDisk>>,
        >,
        /// External network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        pub stateful_external_ips: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulExternalIp>,
            >,
        >,
        /// Internal network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        pub stateful_internal_ips: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulInternalIp>,
            >,
        >,
        /// The status of this managed instance group.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceGroupManagerStatus>,
        >,
        /// The full URL of all target pools to which new
        /// instances in the group are added. Updating the target pools attribute does
        /// not affect existing instances.
        pub target_pools: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The target number of running instances for this managed instance group. This value should always be explicitly set
        /// unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0.
        pub target_size: pulumi_gestalt_rust::Output<i32>,
        /// The target number of stopped instances for this managed instance group.
        pub target_stopped_size: pulumi_gestalt_rust::Output<i32>,
        /// The target number of suspended instances for this managed instance group.
        pub target_suspended_size: pulumi_gestalt_rust::Output<i32>,
        /// The update policy for this managed instance group. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/updating-managed-instance-groups) and [API](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagers/patch).
        pub update_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceGroupManagerUpdatePolicy,
        >,
        /// Application versions managed by this instance group. Each
        /// version deals with a specific instance template, allowing canary release scenarios.
        /// Structure is documented below.
        pub versions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceGroupManagerVersion>,
        >,
        /// Whether to wait for all instances to be created/updated before
        /// returning. Note that if this is set to true and the operation does not succeed, this provider will
        /// continue trying until it times out.
        pub wait_for_instances: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When used with `wait_for_instances` it specifies the status to wait for.
        /// When `STABLE` is specified this resource will wait until the instances are stable before returning. When `UPDATED` is
        /// set, it will wait for the version target to be reached and any per instance configs to be effective as well as all
        /// instances to be stable before returning. The possible values are `STABLE` and `UPDATED`
        pub wait_for_instances_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone that instances in this group should be created
        /// in.
        ///
        /// - - -
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceGroupManagerArgs,
    ) -> InstanceGroupManagerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let all_instances_config_binding = args.all_instances_config.get_output(context);
        let auto_healing_policies_binding = args
            .auto_healing_policies
            .get_output(context);
        let base_instance_name_binding = args.base_instance_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let instance_lifecycle_policy_binding = args
            .instance_lifecycle_policy
            .get_output(context);
        let list_managed_instances_results_binding = args
            .list_managed_instances_results
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let named_ports_binding = args.named_ports.get_output(context);
        let params_binding = args.params.get_output(context);
        let project_binding = args.project.get_output(context);
        let standby_policy_binding = args.standby_policy.get_output(context);
        let stateful_disks_binding = args.stateful_disks.get_output(context);
        let stateful_external_ips_binding = args
            .stateful_external_ips
            .get_output(context);
        let stateful_internal_ips_binding = args
            .stateful_internal_ips
            .get_output(context);
        let target_pools_binding = args.target_pools.get_output(context);
        let target_size_binding = args.target_size.get_output(context);
        let target_stopped_size_binding = args.target_stopped_size.get_output(context);
        let target_suspended_size_binding = args
            .target_suspended_size
            .get_output(context);
        let update_policy_binding = args.update_policy.get_output(context);
        let versions_binding = args.versions.get_output(context);
        let wait_for_instances_binding = args.wait_for_instances.get_output(context);
        let wait_for_instances_status_binding = args
            .wait_for_instances_status
            .get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroupManager:InstanceGroupManager".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allInstancesConfig".into(),
                    value: &all_instances_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoHealingPolicies".into(),
                    value: &auto_healing_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseInstanceName".into(),
                    value: &base_instance_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceLifecyclePolicy".into(),
                    value: &instance_lifecycle_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listManagedInstancesResults".into(),
                    value: &list_managed_instances_results_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namedPorts".into(),
                    value: &named_ports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "params".into(),
                    value: &params_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "standbyPolicy".into(),
                    value: &standby_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statefulDisks".into(),
                    value: &stateful_disks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statefulExternalIps".into(),
                    value: &stateful_external_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statefulInternalIps".into(),
                    value: &stateful_internal_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetPools".into(),
                    value: &target_pools_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetSize".into(),
                    value: &target_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetStoppedSize".into(),
                    value: &target_stopped_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetSuspendedSize".into(),
                    value: &target_suspended_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updatePolicy".into(),
                    value: &update_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versions".into(),
                    value: &versions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForInstances".into(),
                    value: &wait_for_instances_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForInstancesStatus".into(),
                    value: &wait_for_instances_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceGroupManagerResult {
            id: o.get_field("id"),
            all_instances_config: o.get_field("allInstancesConfig"),
            auto_healing_policies: o.get_field("autoHealingPolicies"),
            base_instance_name: o.get_field("baseInstanceName"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            instance_group: o.get_field("instanceGroup"),
            instance_group_manager_id: o.get_field("instanceGroupManagerId"),
            instance_lifecycle_policy: o.get_field("instanceLifecyclePolicy"),
            list_managed_instances_results: o.get_field("listManagedInstancesResults"),
            name: o.get_field("name"),
            named_ports: o.get_field("namedPorts"),
            operation: o.get_field("operation"),
            params: o.get_field("params"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            standby_policy: o.get_field("standbyPolicy"),
            stateful_disks: o.get_field("statefulDisks"),
            stateful_external_ips: o.get_field("statefulExternalIps"),
            stateful_internal_ips: o.get_field("statefulInternalIps"),
            statuses: o.get_field("statuses"),
            target_pools: o.get_field("targetPools"),
            target_size: o.get_field("targetSize"),
            target_stopped_size: o.get_field("targetStoppedSize"),
            target_suspended_size: o.get_field("targetSuspendedSize"),
            update_policy: o.get_field("updatePolicy"),
            versions: o.get_field("versions"),
            wait_for_instances: o.get_field("waitForInstances"),
            wait_for_instances_status: o.get_field("waitForInstancesStatus"),
            zone: o.get_field("zone"),
        }
    }
}
