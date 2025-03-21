/// Provides an Application AutoScaling ScheduledAction resource.
///
/// ## Example Usage
///
/// ### DynamoDB Table Autoscaling
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dynamodb = target::create(
///         "dynamodb",
///         TargetArgs::builder()
///             .max_capacity(100)
///             .min_capacity(5)
///             .resource_id("table/tableName")
///             .scalable_dimension("dynamodb:table:ReadCapacityUnits")
///             .service_namespace("dynamodb")
///             .build_struct(),
///     );
///     let dynamodbScheduledAction = scheduled_action::create(
///         "dynamodbScheduledAction",
///         ScheduledActionArgs::builder()
///             .name("dynamodb")
///             .resource_id("${dynamodb.resourceId}")
///             .scalable_dimension("${dynamodb.scalableDimension}")
///             .scalable_target_action(
///                 ScheduledActionScalableTargetAction::builder()
///                     .maxCapacity(200)
///                     .minCapacity(1)
///                     .build_struct(),
///             )
///             .schedule("at(2006-01-02T15:04:05)")
///             .service_namespace("${dynamodb.serviceNamespace}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### ECS Service Autoscaling
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ecs = target::create(
///         "ecs",
///         TargetArgs::builder()
///             .max_capacity(4)
///             .min_capacity(1)
///             .resource_id("service/clusterName/serviceName")
///             .scalable_dimension("ecs:service:DesiredCount")
///             .service_namespace("ecs")
///             .build_struct(),
///     );
///     let ecsScheduledAction = scheduled_action::create(
///         "ecsScheduledAction",
///         ScheduledActionArgs::builder()
///             .name("ecs")
///             .resource_id("${ecs.resourceId}")
///             .scalable_dimension("${ecs.scalableDimension}")
///             .scalable_target_action(
///                 ScheduledActionScalableTargetAction::builder()
///                     .maxCapacity(10)
///                     .minCapacity(1)
///                     .build_struct(),
///             )
///             .schedule("at(2006-01-02T15:04:05)")
///             .service_namespace("${ecs.serviceNamespace}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scheduled_action {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledActionArgs {
        /// Date and time for the scheduled action to end in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the scheduled action.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the resource associated with the scheduled action. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Scalable dimension. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs:service:DesiredCount
        #[builder(into)]
        pub scalable_dimension: pulumi_gestalt_rust::InputOrOutput<String>,
        /// New minimum and maximum capacity. You can set both values or just one. See below
        #[builder(into)]
        pub scalable_target_action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appautoscaling::ScheduledActionScalableTargetAction,
        >,
        /// Schedule for this action. The following formats are supported: At expressions - at(yyyy-mm-ddThh:mm:ss), Rate expressions - rate(valueunit), Cron expressions - cron(fields). Times for at expressions and cron expressions are evaluated using the time zone configured in `timezone`. Documentation can be found in the `Timezone` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        #[builder(into)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Namespace of the AWS service. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs
        #[builder(into)]
        pub service_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Date and time for the scheduled action to start in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Time zone used when setting a scheduled action by using an at or cron expression. Does not affect timezone for `start_time` and `end_time`. Valid values are the [canonical names of the IANA time zones supported by Joda-Time](https://www.joda.org/joda-time/timezones.html), such as `Etc/GMT+9` or `Pacific/Tahiti`. Default is `UTC`.
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScheduledActionResult {
        /// ARN of the scheduled action.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time for the scheduled action to end in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the scheduled action.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the resource associated with the scheduled action. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Scalable dimension. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs:service:DesiredCount
        pub scalable_dimension: pulumi_gestalt_rust::Output<String>,
        /// New minimum and maximum capacity. You can set both values or just one. See below
        pub scalable_target_action: pulumi_gestalt_rust::Output<
            super::super::types::appautoscaling::ScheduledActionScalableTargetAction,
        >,
        /// Schedule for this action. The following formats are supported: At expressions - at(yyyy-mm-ddThh:mm:ss), Rate expressions - rate(valueunit), Cron expressions - cron(fields). Times for at expressions and cron expressions are evaluated using the time zone configured in `timezone`. Documentation can be found in the `Timezone` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        pub schedule: pulumi_gestalt_rust::Output<String>,
        /// Namespace of the AWS service. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs
        pub service_namespace: pulumi_gestalt_rust::Output<String>,
        /// Date and time for the scheduled action to start in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        pub start_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time zone used when setting a scheduled action by using an at or cron expression. Does not affect timezone for `start_time` and `end_time`. Valid values are the [canonical names of the IANA time zones supported by Joda-Time](https://www.joda.org/joda-time/timezones.html), such as `Etc/GMT+9` or `Pacific/Tahiti`. Default is `UTC`.
        pub timezone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScheduledActionArgs,
    ) -> ScheduledActionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let end_time_binding = args.end_time.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let scalable_dimension_binding = args.scalable_dimension.get_output(context);
        let scalable_target_action_binding = args
            .scalable_target_action
            .get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let service_namespace_binding = args.service_namespace.get_output(context);
        let start_time_binding = args.start_time.get_output(context);
        let timezone_binding = args.timezone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appautoscaling/scheduledAction:ScheduledAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalableDimension".into(),
                    value: &scalable_dimension_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalableTargetAction".into(),
                    value: &scalable_target_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceNamespace".into(),
                    value: &service_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduledActionResult {
            arn: o.get_field("arn"),
            end_time: o.get_field("endTime"),
            name: o.get_field("name"),
            resource_id: o.get_field("resourceId"),
            scalable_dimension: o.get_field("scalableDimension"),
            scalable_target_action: o.get_field("scalableTargetAction"),
            schedule: o.get_field("schedule"),
            service_namespace: o.get_field("serviceNamespace"),
            start_time: o.get_field("startTime"),
            timezone: o.get_field("timezone"),
        }
    }
}
