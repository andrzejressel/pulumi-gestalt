/// Manages a Trigger Schedule inside a Azure Data Factory.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePipeline = pipeline::create(
///         "examplePipeline",
///         PipelineArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleTriggerSchedule = trigger_schedule::create(
///         "exampleTriggerSchedule",
///         TriggerScheduleArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .frequency("Day")
///             .interval(5)
///             .name("example")
///             .pipeline_name("${examplePipeline.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Schedule Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerSchedule:TriggerSchedule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerScheduleArgs {
        /// Specifies if the Data Factory Schedule Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of tags that can be used for describing the Data Factory Schedule Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Schedule Trigger's description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The time the Schedule Trigger should end. The time will be represented in UTC.
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The trigger frequency. Valid values include `Minute`, `Hour`, `Day`, `Week`, `Month`. Defaults to `Minute`.
        #[builder(into, default)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The interval for how often the trigger occurs. This defaults to `1`.
        #[builder(into, default)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Data Factory Schedule Trigger. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Data Factory Pipeline name that the trigger will act on.
        #[builder(into, default)]
        pub pipeline_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The pipeline parameters that the trigger will act upon.
        #[builder(into, default)]
        pub pipeline_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `pipeline` block as defined below.
        #[builder(into, default)]
        pub pipelines: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::TriggerSchedulePipeline>>,
        >,
        /// A `schedule` block as defined below, which further specifies the recurrence schedule for the trigger. A schedule is capable of limiting or increasing the number of trigger executions specified by the `frequency` and `interval` properties.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::TriggerScheduleSchedule>,
        >,
        /// The time the Schedule Trigger will start. This defaults to the current time. The time will be represented in UTC.
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The timezone of the start/end time.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerScheduleResult {
        /// Specifies if the Data Factory Schedule Trigger is activated. Defaults to `true`.
        pub activated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of tags that can be used for describing the Data Factory Schedule Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The Schedule Trigger's description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time the Schedule Trigger should end. The time will be represented in UTC.
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The trigger frequency. Valid values include `Minute`, `Hour`, `Day`, `Week`, `Month`. Defaults to `Minute`.
        pub frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The interval for how often the trigger occurs. This defaults to `1`.
        pub interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Schedule Trigger. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Data Factory Pipeline name that the trigger will act on.
        pub pipeline_name: pulumi_gestalt_rust::Output<String>,
        /// The pipeline parameters that the trigger will act upon.
        pub pipeline_parameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `pipeline` block as defined below.
        pub pipelines: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datafactory::TriggerSchedulePipeline>,
        >,
        /// A `schedule` block as defined below, which further specifies the recurrence schedule for the trigger. A schedule is capable of limiting or increasing the number of trigger executions specified by the `frequency` and `interval` properties.
        pub schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::TriggerScheduleSchedule>,
        >,
        /// The time the Schedule Trigger will start. This defaults to the current time. The time will be represented in UTC.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The timezone of the start/end time.
        pub time_zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerScheduleArgs,
    ) -> TriggerScheduleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activated_binding = args.activated.get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let end_time_binding = args.end_time.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let name_binding = args.name.get_output(context);
        let pipeline_name_binding = args.pipeline_name.get_output(context);
        let pipeline_parameters_binding = args.pipeline_parameters.get_output(context);
        let pipelines_binding = args.pipelines.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let start_time_binding = args.start_time.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/triggerSchedule:TriggerSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activated".into(),
                    value: &activated_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interval".into(),
                    value: &interval_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineName".into(),
                    value: &pipeline_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineParameters".into(),
                    value: &pipeline_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelines".into(),
                    value: &pipelines_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerScheduleResult {
            activated: o.get_field("activated"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            end_time: o.get_field("endTime"),
            frequency: o.get_field("frequency"),
            interval: o.get_field("interval"),
            name: o.get_field("name"),
            pipeline_name: o.get_field("pipelineName"),
            pipeline_parameters: o.get_field("pipelineParameters"),
            pipelines: o.get_field("pipelines"),
            schedule: o.get_field("schedule"),
            start_time: o.get_field("startTime"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
