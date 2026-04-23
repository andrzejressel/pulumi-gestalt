/// Manages a Custom Event Trigger inside an Azure Data Factory.
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
///   exampleTopic:
///     type: azure:eventgrid:Topic
///     name: example
///     properties:
///       name: example-topic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleTriggerCustomEvent:
///     type: azure:datafactory:TriggerCustomEvent
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       eventgridTopicId: ${exampleTopic.id}
///       events:
///         - event1
///         - event2
///       subjectBeginsWith: abc
///       subjectEndsWith: xyz
///       annotations:
///         - example1
///         - example2
///         - example3
///       description: example description
///       pipelines:
///         - name: ${examplePipeline.name}
///           parameters:
///             Env: Prod
///       additionalProperties:
///         foo: foo1
///         bar: bar2
/// ```
///
/// ## Import
///
/// Data Factory Custom Event Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerCustomEvent:TriggerCustomEvent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod trigger_custom_event {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerCustomEventArgs {
        /// Specifies if the Data Factory Custom Event Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_gestalt_rust::Input<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::Input<String>,
        /// The description for the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of Event Grid Topic in which event will be listened. Changing this forces a new resource.
        #[builder(into)]
        pub eventgrid_topic_id: pulumi_gestalt_rust::Input<String>,
        /// List of events that will fire this trigger. At least one event must be specified.
        #[builder(into)]
        pub events: pulumi_gestalt_rust::Input<Vec<String>>,
        /// Specifies the name of the Data Factory Custom Event Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// One or more `pipeline` blocks as defined below.
        #[builder(into)]
        pub pipelines: pulumi_gestalt_rust::Input<
            Vec<super::super::types::datafactory::TriggerCustomEventPipeline>,
        >,
        /// The pattern that event subject starts with for trigger to fire.
        #[builder(into, default)]
        pub subject_begins_with: pulumi_gestalt_rust::Input<Option<String>>,
        /// The pattern that event subject ends with for trigger to fire.
        #[builder(into, default)]
        pub subject_ends_with: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerCustomEventResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the Data Factory Custom Event Trigger is activated. Defaults to `true`.
        pub activated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Custom Event Trigger.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Custom Event Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Custom Event Trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of Event Grid Topic in which event will be listened. Changing this forces a new resource.
        pub eventgrid_topic_id: pulumi_gestalt_rust::Output<String>,
        /// List of events that will fire this trigger. At least one event must be specified.
        pub events: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the name of the Data Factory Custom Event Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `pipeline` blocks as defined below.
        pub pipelines: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datafactory::TriggerCustomEventPipeline>,
        >,
        /// The pattern that event subject starts with for trigger to fire.
        pub subject_begins_with: pulumi_gestalt_rust::Output<Option<String>>,
        /// The pattern that event subject ends with for trigger to fire.
        pub subject_ends_with: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerCustomEventArgs,
    ) -> TriggerCustomEventResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerCustomEventArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TriggerCustomEventResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerCustomEventArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TriggerCustomEventResult {
        let activated_binding = args.activated.get_output(ctx);
        let additional_properties_binding = args.additional_properties.get_output(ctx);
        let annotations_binding = args.annotations.get_output(ctx);
        let data_factory_id_binding = args.data_factory_id.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let eventgrid_topic_id_binding = args.eventgrid_topic_id.get_output(ctx);
        let events_binding = args.events.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let pipelines_binding = args.pipelines.get_output(ctx);
        let subject_begins_with_binding = args.subject_begins_with.get_output(ctx);
        let subject_ends_with_binding = args.subject_ends_with.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/triggerCustomEvent:TriggerCustomEvent".into(),
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
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventgridTopicId".into(),
                    value: &eventgrid_topic_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "events".into(),
                    value: &events_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelines".into(),
                    value: &pipelines_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subjectBeginsWith".into(),
                    value: &subject_begins_with_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subjectEndsWith".into(),
                    value: &subject_ends_with_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TriggerCustomEventResult {
            id: o.get_id(),
            urn: o.get_urn(),
            activated: o.get_field("activated"),
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            eventgrid_topic_id: o.get_field("eventgridTopicId"),
            events: o.get_field("events"),
            name: o.get_field("name"),
            pipelines: o.get_field("pipelines"),
            subject_begins_with: o.get_field("subjectBeginsWith"),
            subject_ends_with: o.get_field("subjectEndsWith"),
        }
    }
}
