/// Manages a Stream Analytics Output to a ServiceBus Queue.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Standard
///   exampleQueue:
///     type: azure:servicebus:Queue
///     name: example
///     properties:
///       name: example-queue
///       namespaceId: ${exampleNamespace.id}
///       enablePartitioning: true
///   exampleOutputServiceBusQueue:
///     type: azure:streamanalytics:OutputServiceBusQueue
///     name: example
///     properties:
///       name: blob-storage-output
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       queueName: ${exampleQueue.name}
///       servicebusNamespace: ${exampleNamespace.name}
///       sharedAccessPolicyKey: ${exampleNamespace.defaultPrimaryKey}
///       sharedAccessPolicyName: RootManageSharedAccessKey
///       serialization:
///         type: Csv
///         format: Array
/// variables:
///   example:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Stream Analytics Output ServiceBus Queue's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputServiceBusQueue:OutputServiceBusQueue example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod output_service_bus_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputServiceBusQueueArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of property columns to add to the Service Bus Queue output.
        #[builder(into, default)]
        pub property_columns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Service Bus Queue.
        #[builder(into)]
        pub queue_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::OutputServiceBusQueueSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        #[builder(into)]
        pub servicebus_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The shared access policy key for the specified shared access policy. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A key-value pair of system property columns that will be attached to the outgoing messages for the Service Bus Queue Output.
        ///
        /// > **NOTE:** The acceptable keys are `ContentType`, `CorrelationId`, `Label`, `MessageId`, `PartitionKey`, `ReplyTo`, `ReplyToSessionId`, `ScheduledEnqueueTimeUtc`, `SessionId`, `TimeToLive` and `To`.
        #[builder(into, default)]
        pub system_property_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct OutputServiceBusQueueResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of property columns to add to the Service Bus Queue output.
        pub property_columns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the Service Bus Queue.
        pub queue_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::OutputServiceBusQueueSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub servicebus_namespace: pulumi_gestalt_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy. Required if `authentication_mode` is `ConnectionString`.
        pub shared_access_policy_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required if `authentication_mode` is `ConnectionString`.
        pub shared_access_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
        /// A key-value pair of system property columns that will be attached to the outgoing messages for the Service Bus Queue Output.
        ///
        /// > **NOTE:** The acceptable keys are `ContentType`, `CorrelationId`, `Label`, `MessageId`, `PartitionKey`, `ReplyTo`, `ReplyToSessionId`, `ScheduledEnqueueTimeUtc`, `SessionId`, `TimeToLive` and `To`.
        pub system_property_columns: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OutputServiceBusQueueArgs,
    ) -> OutputServiceBusQueueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_mode_binding = args.authentication_mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let property_columns_binding = args.property_columns.get_output(context);
        let queue_name_binding = args.queue_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let serialization_binding = args.serialization.get_output(context);
        let servicebus_namespace_binding = args.servicebus_namespace.get_output(context);
        let shared_access_policy_key_binding = args
            .shared_access_policy_key
            .get_output(context);
        let shared_access_policy_name_binding = args
            .shared_access_policy_name
            .get_output(context);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context);
        let system_property_columns_binding = args
            .system_property_columns
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputServiceBusQueue:OutputServiceBusQueue"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "propertyColumns".into(),
                    value: &property_columns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueName".into(),
                    value: &queue_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serialization".into(),
                    value: &serialization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicebusNamespace".into(),
                    value: &servicebus_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccessPolicyKey".into(),
                    value: &shared_access_policy_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccessPolicyName".into(),
                    value: &shared_access_policy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "systemPropertyColumns".into(),
                    value: &system_property_columns_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OutputServiceBusQueueResult {
            authentication_mode: o.get_field("authenticationMode"),
            name: o.get_field("name"),
            property_columns: o.get_field("propertyColumns"),
            queue_name: o.get_field("queueName"),
            resource_group_name: o.get_field("resourceGroupName"),
            serialization: o.get_field("serialization"),
            servicebus_namespace: o.get_field("servicebusNamespace"),
            shared_access_policy_key: o.get_field("sharedAccessPolicyKey"),
            shared_access_policy_name: o.get_field("sharedAccessPolicyName"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
            system_property_columns: o.get_field("systemPropertyColumns"),
        }
    }
}
