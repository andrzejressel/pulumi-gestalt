#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// Specify the exact name of the desired autoscaling group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        /// ARN of the Auto Scaling group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// One or more Availability Zones for the group.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub default_cooldown: pulumi_gestalt_rust::Output<i32>,
        /// Desired size of the group.
        pub desired_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The unit of measurement for the value returned for `desired_capacity`.
        pub desired_capacity_type: pulumi_gestalt_rust::Output<String>,
        /// List of metrics enabled for collection.
        pub enabled_metrics: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service.
        pub health_check_grace_period: pulumi_gestalt_rust::Output<i32>,
        /// Service to use for the health checks. The valid values are EC2 and ELB.
        pub health_check_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Instance maintenance policy for the group.
        pub instance_maintenance_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::autoscaling::GetGroupInstanceMaintenancePolicy,
            >,
        >,
        /// The name of the associated launch configuration.
        pub launch_configuration: pulumi_gestalt_rust::Output<String>,
        /// List of launch templates along with the overrides.
        pub launch_templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupLaunchTemplate>,
        >,
        /// One or more load balancers associated with the group.
        pub load_balancers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Maximum amount of time, in seconds, that an instance can be in service.
        pub max_instance_lifetime: pulumi_gestalt_rust::Output<i32>,
        /// Maximum size of the group.
        pub max_size: pulumi_gestalt_rust::Output<i32>,
        /// Minimum number of instances to maintain in the warm pool.
        pub min_size: pulumi_gestalt_rust::Output<i32>,
        /// List of mixed instances policy objects for the group.
        pub mixed_instances_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupMixedInstancesPolicy>,
        >,
        /// Name of the Auto Scaling Group.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub new_instances_protected_from_scale_in: pulumi_gestalt_rust::Output<bool>,
        /// Name of the placement group into which to launch your instances, if any. For more information, see Placement Groups (http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html) in the Amazon Elastic Compute Cloud User Guide.
        pub placement_group: pulumi_gestalt_rust::Output<String>,
        /// Predicted capacity of the group.
        pub predicted_capacity: pulumi_gestalt_rust::Output<i32>,
        /// ARN of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf.
        pub service_linked_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Current state of the group when DeleteAutoScalingGroup is in progress.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// List of processes suspended processes for the Auto Scaling Group.
        pub suspended_processes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of tags for the group.
        pub tags: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupTag>,
        >,
        /// ARNs of the target groups for your load balancer.
        pub target_group_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The termination policies for the group.
        pub termination_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Traffic sources.
        pub traffic_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupTrafficSource>,
        >,
        /// VPC ID for the group.
        pub vpc_zone_identifier: pulumi_gestalt_rust::Output<String>,
        /// Current size of the warm pool.
        pub warm_pool_size: pulumi_gestalt_rust::Output<i32>,
        /// List of warm pool configuration objects.
        pub warm_pools: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupWarmPool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:autoscaling/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupResult {
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            default_cooldown: o.get_field("defaultCooldown"),
            desired_capacity: o.get_field("desiredCapacity"),
            desired_capacity_type: o.get_field("desiredCapacityType"),
            enabled_metrics: o.get_field("enabledMetrics"),
            health_check_grace_period: o.get_field("healthCheckGracePeriod"),
            health_check_type: o.get_field("healthCheckType"),
            id: o.get_field("id"),
            instance_maintenance_policies: o.get_field("instanceMaintenancePolicies"),
            launch_configuration: o.get_field("launchConfiguration"),
            launch_templates: o.get_field("launchTemplates"),
            load_balancers: o.get_field("loadBalancers"),
            max_instance_lifetime: o.get_field("maxInstanceLifetime"),
            max_size: o.get_field("maxSize"),
            min_size: o.get_field("minSize"),
            mixed_instances_policies: o.get_field("mixedInstancesPolicies"),
            name: o.get_field("name"),
            new_instances_protected_from_scale_in: o
                .get_field("newInstancesProtectedFromScaleIn"),
            placement_group: o.get_field("placementGroup"),
            predicted_capacity: o.get_field("predictedCapacity"),
            service_linked_role_arn: o.get_field("serviceLinkedRoleArn"),
            status: o.get_field("status"),
            suspended_processes: o.get_field("suspendedProcesses"),
            tags: o.get_field("tags"),
            target_group_arns: o.get_field("targetGroupArns"),
            termination_policies: o.get_field("terminationPolicies"),
            traffic_sources: o.get_field("trafficSources"),
            vpc_zone_identifier: o.get_field("vpcZoneIdentifier"),
            warm_pool_size: o.get_field("warmPoolSize"),
            warm_pools: o.get_field("warmPools"),
        }
    }
}
