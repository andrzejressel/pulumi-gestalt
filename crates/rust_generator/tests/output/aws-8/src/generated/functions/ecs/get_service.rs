#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// ARN of the ECS Cluster
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the ECS Service
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// ARN of the ECS Service
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub availability_zone_rebalancing: pulumi_gestalt_rust::Output<String>,
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// Number of tasks for the ECS Service
        pub desired_count: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Launch type for the ECS Service
        pub launch_type: pulumi_gestalt_rust::Output<String>,
        /// Scheduling strategy for the ECS Service
        pub scheduling_strategy: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Family for the latest ACTIVE revision or full ARN of the task definition.
        pub task_definition: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_arn_binding = args.cluster_arn.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecs/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            arn: o.get_field("arn"),
            availability_zone_rebalancing: o.get_field("availabilityZoneRebalancing"),
            cluster_arn: o.get_field("clusterArn"),
            desired_count: o.get_field("desiredCount"),
            id: o.get_field("id"),
            launch_type: o.get_field("launchType"),
            scheduling_strategy: o.get_field("schedulingStrategy"),
            service_name: o.get_field("serviceName"),
            tags: o.get_field("tags"),
            task_definition: o.get_field("taskDefinition"),
        }
    }
}
