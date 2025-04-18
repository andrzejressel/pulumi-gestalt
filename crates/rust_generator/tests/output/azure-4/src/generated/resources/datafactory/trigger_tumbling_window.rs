/// Manages a Tumbling Window Trigger inside an Azure Data Factory.
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
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   examplePipeline:
///     type: azure:datafactory:Pipeline
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///   exampleTriggerTumblingWindow:
///     type: azure:datafactory:TriggerTumblingWindow
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       startTime: 2022-09-21T00:00:00Z
///       endTime: 2022-09-21T08:00:00Z
///       frequency: Minute
///       interval: 15
///       delay: 16:00:00
///       annotations:
///         - example1
///         - example2
///         - example3
///       description: example description
///       retry:
///         count: 1
///         interval: 30
///       pipeline:
///         name: ${examplePipeline.name}
///         parameters:
///           Env: Prod
///       triggerDependencies:
///         - size: 24:00:00
///           offset: -24:00:00
///       additionalProperties:
///         foo: value1
///         bar: value2
/// ```
///
/// ## Import
///
/// Data Factory Tumbling Window Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerTumblingWindow:TriggerTumblingWindow example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger_tumbling_window {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerTumblingWindowArgs {
        /// Specifies if the Data Factory Tumbling Window Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies how long the trigger waits before triggering new run. formatted as an `D.HH:MM:SS`.
        #[builder(into, default)]
        pub delay: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the end time of Tumbling Window, formatted as an RFC3339 string.
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the frequency of Tumbling Window. Possible values are `Hour`, `Minute` and `Month`. Changing this forces a new resource.
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the interval of Tumbling Window. Changing this forces a new resource.
        #[builder(into)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The max number for simultaneous trigger run fired by Tumbling Window. Possible values are between `1` and `50`. Defaults to `50`.
        #[builder(into, default)]
        pub max_concurrency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Data Factory Tumbling Window Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `pipeline` block as defined below.
        #[builder(into)]
        pub pipeline: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datafactory::TriggerTumblingWindowPipeline,
        >,
        /// A `retry` block as defined below.
        #[builder(into, default)]
        pub retry: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::TriggerTumblingWindowRetry>,
        >,
        /// Specifies the start time of Tumbling Window, formatted as an RFC3339 string. Changing this forces a new resource.
        #[builder(into)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `trigger_dependency` block as defined below.
        #[builder(into, default)]
        pub trigger_dependencies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::datafactory::TriggerTumblingWindowTriggerDependency,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerTumblingWindowResult {
        /// Specifies if the Data Factory Tumbling Window Trigger is activated. Defaults to `true`.
        pub activated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Tumbling Window Trigger.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Tumbling Window Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies how long the trigger waits before triggering new run. formatted as an `D.HH:MM:SS`.
        pub delay: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for the Data Factory Tumbling Window Trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the end time of Tumbling Window, formatted as an RFC3339 string.
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the frequency of Tumbling Window. Possible values are `Hour`, `Minute` and `Month`. Changing this forces a new resource.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// Specifies the interval of Tumbling Window. Changing this forces a new resource.
        pub interval: pulumi_gestalt_rust::Output<i32>,
        /// The max number for simultaneous trigger run fired by Tumbling Window. Possible values are between `1` and `50`. Defaults to `50`.
        pub max_concurrency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Tumbling Window Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `pipeline` block as defined below.
        pub pipeline: pulumi_gestalt_rust::Output<
            super::super::types::datafactory::TriggerTumblingWindowPipeline,
        >,
        /// A `retry` block as defined below.
        pub retry: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::TriggerTumblingWindowRetry>,
        >,
        /// Specifies the start time of Tumbling Window, formatted as an RFC3339 string. Changing this forces a new resource.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// One or more `trigger_dependency` block as defined below.
        pub trigger_dependencies: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::datafactory::TriggerTumblingWindowTriggerDependency,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerTumblingWindowArgs,
    ) -> TriggerTumblingWindowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activated_binding = args.activated.get_output(context);
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let delay_binding = args.delay.get_output(context);
        let description_binding = args.description.get_output(context);
        let end_time_binding = args.end_time.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let max_concurrency_binding = args.max_concurrency.get_output(context);
        let name_binding = args.name.get_output(context);
        let pipeline_binding = args.pipeline.get_output(context);
        let retry_binding = args.retry.get_output(context);
        let start_time_binding = args.start_time.get_output(context);
        let trigger_dependencies_binding = args.trigger_dependencies.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/triggerTumblingWindow:TriggerTumblingWindow"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activated".into(),
                    value: &activated_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding.drop_type(),
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
                    name: "delay".into(),
                    value: &delay_binding.drop_type(),
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
                    name: "maxConcurrency".into(),
                    value: &max_concurrency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipeline".into(),
                    value: &pipeline_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retry".into(),
                    value: &retry_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerDependencies".into(),
                    value: &trigger_dependencies_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerTumblingWindowResult {
            activated: o.get_field("activated"),
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            delay: o.get_field("delay"),
            description: o.get_field("description"),
            end_time: o.get_field("endTime"),
            frequency: o.get_field("frequency"),
            interval: o.get_field("interval"),
            max_concurrency: o.get_field("maxConcurrency"),
            name: o.get_field("name"),
            pipeline: o.get_field("pipeline"),
            retry: o.get_field("retry"),
            start_time: o.get_field("startTime"),
            trigger_dependencies: o.get_field("triggerDependencies"),
        }
    }
}
