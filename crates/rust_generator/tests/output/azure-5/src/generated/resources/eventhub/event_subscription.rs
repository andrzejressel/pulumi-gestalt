/// Manages an EventGrid Event Subscription
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: exampleasa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleQueue:
///     type: azure:storage:Queue
///     name: example
///     properties:
///       name: example-astq
///       storageAccountName: ${exampleAccount.name}
///   exampleEventSubscription:
///     type: azure:eventgrid:EventSubscription
///     name: example
///     properties:
///       name: example-aees
///       scope: ${example.id}
///       storageQueueEndpoint:
///         storageAccountId: ${exampleAccount.id}
///         queueName: ${exampleQueue.name}
/// ```
///
/// ## Import
///
/// EventGrid Event Subscription's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/eventSubscription:EventSubscription eventSubscription1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1/providers/Microsoft.EventGrid/eventSubscriptions/eventSubscription1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSubscriptionArgs {
        /// A `advanced_filter` block as defined below.
        #[builder(into, default)]
        pub advanced_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionAdvancedFilter>,
        >,
        /// Specifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
        #[builder(into, default)]
        pub advanced_filtering_on_arrays_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `azure_function_endpoint` block as defined below.
        #[builder(into, default)]
        pub azure_function_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionAzureFunctionEndpoint>,
        >,
        /// A `dead_letter_identity` block as defined below.
        ///
        /// > **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
        #[builder(into, default)]
        pub dead_letter_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionDeadLetterIdentity>,
        >,
        /// A `delivery_identity` block as defined below.
        #[builder(into, default)]
        pub delivery_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionDeliveryIdentity>,
        >,
        /// One or more `delivery_property` blocks as defined below.
        #[builder(into, default)]
        pub delivery_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::eventhub::EventSubscriptionDeliveryProperty>>,
        >,
        /// Specifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub event_delivery_schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the id where the Event Hub is located.
        #[builder(into, default)]
        pub eventhub_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
        #[builder(into, default)]
        pub expiration_time_utc: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the id where the Hybrid Connection is located.
        #[builder(into, default)]
        pub hybrid_connection_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of applicable event types that need to be part of the event subscription.
        #[builder(into, default)]
        pub included_event_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of labels to assign to the event subscription.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the name of the EventGrid Event Subscription resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `retry_policy` block as defined below.
        #[builder(into, default)]
        pub retry_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionRetryPolicy>,
        >,
        /// Specifies the scope at which the EventGrid Event Subscription should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the id where the Service Bus Queue is located.
        #[builder(into, default)]
        pub service_bus_queue_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the id where the Service Bus Topic is located.
        #[builder(into, default)]
        pub service_bus_topic_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `storage_blob_dead_letter_destination` block as defined below.
        #[builder(into, default)]
        pub storage_blob_dead_letter_destination: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventhub::EventSubscriptionStorageBlobDeadLetterDestination,
            >,
        >,
        /// A `storage_queue_endpoint` block as defined below.
        #[builder(into, default)]
        pub storage_queue_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionStorageQueueEndpoint>,
        >,
        /// A `subject_filter` block as defined below.
        #[builder(into, default)]
        pub subject_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionSubjectFilter>,
        >,
        /// A `webhook_endpoint` block as defined below.
        ///
        /// > **NOTE:** One of `eventhub_endpoint_id`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint`, `webhook_endpoint` or `azure_function_endpoint` must be specified.
        #[builder(into, default)]
        pub webhook_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventSubscriptionWebhookEndpoint>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSubscriptionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `advanced_filter` block as defined below.
        pub advanced_filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionAdvancedFilter>,
        >,
        /// Specifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
        pub advanced_filtering_on_arrays_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// An `azure_function_endpoint` block as defined below.
        pub azure_function_endpoint: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionAzureFunctionEndpoint>,
        >,
        /// A `dead_letter_identity` block as defined below.
        ///
        /// > **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
        pub dead_letter_identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionDeadLetterIdentity>,
        >,
        /// A `delivery_identity` block as defined below.
        pub delivery_identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionDeliveryIdentity>,
        >,
        /// One or more `delivery_property` blocks as defined below.
        pub delivery_properties: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::eventhub::EventSubscriptionDeliveryProperty>>,
        >,
        /// Specifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        pub event_delivery_schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id where the Event Hub is located.
        pub eventhub_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
        pub expiration_time_utc: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id where the Hybrid Connection is located.
        pub hybrid_connection_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// A list of applicable event types that need to be part of the event subscription.
        pub included_event_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of labels to assign to the event subscription.
        pub labels: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the name of the EventGrid Event Subscription resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `retry_policy` block as defined below.
        pub retry_policy: pulumi_gestalt_rust::Output<
            super::super::types::eventhub::EventSubscriptionRetryPolicy,
        >,
        /// Specifies the scope at which the EventGrid Event Subscription should be created. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// Specifies the id where the Service Bus Queue is located.
        pub service_bus_queue_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id where the Service Bus Topic is located.
        pub service_bus_topic_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `storage_blob_dead_letter_destination` block as defined below.
        pub storage_blob_dead_letter_destination: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventhub::EventSubscriptionStorageBlobDeadLetterDestination,
            >,
        >,
        /// A `storage_queue_endpoint` block as defined below.
        pub storage_queue_endpoint: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionStorageQueueEndpoint>,
        >,
        /// A `subject_filter` block as defined below.
        pub subject_filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionSubjectFilter>,
        >,
        /// A `webhook_endpoint` block as defined below.
        ///
        /// > **NOTE:** One of `eventhub_endpoint_id`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint`, `webhook_endpoint` or `azure_function_endpoint` must be specified.
        pub webhook_endpoint: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventSubscriptionWebhookEndpoint>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventSubscriptionArgs,
    ) -> EventSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let advanced_filter_binding = args.advanced_filter.get_output(context);
        let advanced_filtering_on_arrays_enabled_binding = args
            .advanced_filtering_on_arrays_enabled
            .get_output(context);
        let azure_function_endpoint_binding = args
            .azure_function_endpoint
            .get_output(context);
        let dead_letter_identity_binding = args.dead_letter_identity.get_output(context);
        let delivery_identity_binding = args.delivery_identity.get_output(context);
        let delivery_properties_binding = args.delivery_properties.get_output(context);
        let event_delivery_schema_binding = args
            .event_delivery_schema
            .get_output(context);
        let eventhub_endpoint_id_binding = args.eventhub_endpoint_id.get_output(context);
        let expiration_time_utc_binding = args.expiration_time_utc.get_output(context);
        let hybrid_connection_endpoint_id_binding = args
            .hybrid_connection_endpoint_id
            .get_output(context);
        let included_event_types_binding = args.included_event_types.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let retry_policy_binding = args.retry_policy.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let service_bus_queue_endpoint_id_binding = args
            .service_bus_queue_endpoint_id
            .get_output(context);
        let service_bus_topic_endpoint_id_binding = args
            .service_bus_topic_endpoint_id
            .get_output(context);
        let storage_blob_dead_letter_destination_binding = args
            .storage_blob_dead_letter_destination
            .get_output(context);
        let storage_queue_endpoint_binding = args
            .storage_queue_endpoint
            .get_output(context);
        let subject_filter_binding = args.subject_filter.get_output(context);
        let webhook_endpoint_binding = args.webhook_endpoint.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/eventSubscription:EventSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "advancedFilter".into(),
                    value: &advanced_filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "advancedFilteringOnArraysEnabled".into(),
                    value: &advanced_filtering_on_arrays_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureFunctionEndpoint".into(),
                    value: &azure_function_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterIdentity".into(),
                    value: &dead_letter_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryIdentity".into(),
                    value: &delivery_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryProperties".into(),
                    value: &delivery_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventDeliverySchema".into(),
                    value: &event_delivery_schema_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubEndpointId".into(),
                    value: &eventhub_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expirationTimeUtc".into(),
                    value: &expiration_time_utc_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hybridConnectionEndpointId".into(),
                    value: &hybrid_connection_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includedEventTypes".into(),
                    value: &included_event_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retryPolicy".into(),
                    value: &retry_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceBusQueueEndpointId".into(),
                    value: &service_bus_queue_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceBusTopicEndpointId".into(),
                    value: &service_bus_topic_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageBlobDeadLetterDestination".into(),
                    value: &storage_blob_dead_letter_destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageQueueEndpoint".into(),
                    value: &storage_queue_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subjectFilter".into(),
                    value: &subject_filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webhookEndpoint".into(),
                    value: &webhook_endpoint_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventSubscriptionResult {
            id: o.get_field("id"),
            advanced_filter: o.get_field("advancedFilter"),
            advanced_filtering_on_arrays_enabled: o
                .get_field("advancedFilteringOnArraysEnabled"),
            azure_function_endpoint: o.get_field("azureFunctionEndpoint"),
            dead_letter_identity: o.get_field("deadLetterIdentity"),
            delivery_identity: o.get_field("deliveryIdentity"),
            delivery_properties: o.get_field("deliveryProperties"),
            event_delivery_schema: o.get_field("eventDeliverySchema"),
            eventhub_endpoint_id: o.get_field("eventhubEndpointId"),
            expiration_time_utc: o.get_field("expirationTimeUtc"),
            hybrid_connection_endpoint_id: o.get_field("hybridConnectionEndpointId"),
            included_event_types: o.get_field("includedEventTypes"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            retry_policy: o.get_field("retryPolicy"),
            scope: o.get_field("scope"),
            service_bus_queue_endpoint_id: o.get_field("serviceBusQueueEndpointId"),
            service_bus_topic_endpoint_id: o.get_field("serviceBusTopicEndpointId"),
            storage_blob_dead_letter_destination: o
                .get_field("storageBlobDeadLetterDestination"),
            storage_queue_endpoint: o.get_field("storageQueueEndpoint"),
            subject_filter: o.get_field("subjectFilter"),
            webhook_endpoint: o.get_field("webhookEndpoint"),
        }
    }
}
