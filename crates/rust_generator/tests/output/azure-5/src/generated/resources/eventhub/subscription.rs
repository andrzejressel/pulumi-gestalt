/// Manages a ServiceBus Subscription.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-servicebus-subscription
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: tfex-servicebus-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       tags:
///         source: example
///   exampleTopic:
///     type: azure:servicebus:Topic
///     name: example
///     properties:
///       name: tfex_servicebus_topic
///       namespaceId: ${exampleNamespace.id}
///       partitioningEnabled: true
///   exampleSubscription:
///     type: azure:servicebus:Subscription
///     name: example
///     properties:
///       name: tfex_servicebus_subscription
///       topicId: ${exampleTopic.id}
///       maxDeliveryCount: 1
/// ```
///
/// ## Import
///
/// Service Bus Subscriptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/subscription:Subscription example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/topics/sntopic1/subscriptions/sbsub1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionArgs {
        /// The idle interval after which the topic is automatically deleted as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `5` minutes or `PT5M`. Defaults to `P10675199DT2H48M5.4775807S`.
        #[builder(into, default)]
        pub auto_delete_on_idle: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean flag which controls whether the Subscription supports batched operations.
        #[builder(into, default)]
        pub batched_operations_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `client_scoped_subscription` block as defined below.
        #[builder(into, default)]
        pub client_scoped_subscription: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::SubscriptionClientScopedSubscription>,
        >,
        /// whether the subscription is scoped to a client id. Defaults to `false`.
        ///
        /// > **NOTE:** Client Scoped Subscription can only be used for JMS subscription (Java Message Service).
        #[builder(into, default)]
        pub client_scoped_subscription_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean flag which controls whether the Subscription has dead letter support on filter evaluation exceptions. Defaults to `true`.
        #[builder(into, default)]
        pub dead_lettering_on_filter_evaluation_error: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean flag which controls whether the Subscription has dead letter support when a message expires.
        #[builder(into, default)]
        pub dead_lettering_on_message_expiration: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Default message timespan to live as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the value used when TimeToLive is not set on a message itself. Defaults to `P10675199DT2H48M5.4775807S`.
        #[builder(into, default)]
        pub default_message_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a Queue or Topic to automatically forward Dead Letter messages to.
        #[builder(into, default)]
        pub forward_dead_lettered_messages_to: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of a Queue or Topic to automatically forward messages to.
        #[builder(into, default)]
        pub forward_to: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The lock duration for the subscription as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The default value is `1` minute or `P0DT0H1M0S` . The maximum value is `5` minutes or `P0DT0H5M0S` . Defaults to `PT1M`.
        #[builder(into, default)]
        pub lock_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of deliveries.
        #[builder(into)]
        pub max_delivery_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the name of the ServiceBus Subscription resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean flag which controls whether this Subscription supports the concept of a session. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub requires_session: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The status of the Subscription. Possible values are `Active`,`ReceiveDisabled`, or `Disabled`. Defaults to `Active`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the ServiceBus Topic to create this Subscription in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub topic_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionResult {
        /// The idle interval after which the topic is automatically deleted as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `5` minutes or `PT5M`. Defaults to `P10675199DT2H48M5.4775807S`.
        pub auto_delete_on_idle: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Subscription supports batched operations.
        pub batched_operations_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `client_scoped_subscription` block as defined below.
        pub client_scoped_subscription: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::SubscriptionClientScopedSubscription>,
        >,
        /// whether the subscription is scoped to a client id. Defaults to `false`.
        ///
        /// > **NOTE:** Client Scoped Subscription can only be used for JMS subscription (Java Message Service).
        pub client_scoped_subscription_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Boolean flag which controls whether the Subscription has dead letter support on filter evaluation exceptions. Defaults to `true`.
        pub dead_lettering_on_filter_evaluation_error: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Boolean flag which controls whether the Subscription has dead letter support when a message expires.
        pub dead_lettering_on_message_expiration: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The Default message timespan to live as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the value used when TimeToLive is not set on a message itself. Defaults to `P10675199DT2H48M5.4775807S`.
        pub default_message_ttl: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of a Queue or Topic to automatically forward Dead Letter messages to.
        pub forward_dead_lettered_messages_to: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The name of a Queue or Topic to automatically forward messages to.
        pub forward_to: pulumi_gestalt_rust::Output<Option<String>>,
        /// The lock duration for the subscription as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The default value is `1` minute or `P0DT0H1M0S` . The maximum value is `5` minutes or `P0DT0H5M0S` . Defaults to `PT1M`.
        pub lock_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum number of deliveries.
        pub max_delivery_count: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the ServiceBus Subscription resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Boolean flag which controls whether this Subscription supports the concept of a session. Changing this forces a new resource to be created.
        pub requires_session: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The status of the Subscription. Possible values are `Active`,`ReceiveDisabled`, or `Disabled`. Defaults to `Active`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the ServiceBus Topic to create this Subscription in. Changing this forces a new resource to be created.
        pub topic_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriptionArgs,
    ) -> SubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_delete_on_idle_binding = args.auto_delete_on_idle.get_output(context);
        let batched_operations_enabled_binding = args
            .batched_operations_enabled
            .get_output(context);
        let client_scoped_subscription_binding = args
            .client_scoped_subscription
            .get_output(context);
        let client_scoped_subscription_enabled_binding = args
            .client_scoped_subscription_enabled
            .get_output(context);
        let dead_lettering_on_filter_evaluation_error_binding = args
            .dead_lettering_on_filter_evaluation_error
            .get_output(context);
        let dead_lettering_on_message_expiration_binding = args
            .dead_lettering_on_message_expiration
            .get_output(context);
        let default_message_ttl_binding = args.default_message_ttl.get_output(context);
        let forward_dead_lettered_messages_to_binding = args
            .forward_dead_lettered_messages_to
            .get_output(context);
        let forward_to_binding = args.forward_to.get_output(context);
        let lock_duration_binding = args.lock_duration.get_output(context);
        let max_delivery_count_binding = args.max_delivery_count.get_output(context);
        let name_binding = args.name.get_output(context);
        let requires_session_binding = args.requires_session.get_output(context);
        let status_binding = args.status.get_output(context);
        let topic_id_binding = args.topic_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/subscription:Subscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoDeleteOnIdle".into(),
                    value: &auto_delete_on_idle_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchedOperationsEnabled".into(),
                    value: &batched_operations_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientScopedSubscription".into(),
                    value: &client_scoped_subscription_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientScopedSubscriptionEnabled".into(),
                    value: &client_scoped_subscription_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetteringOnFilterEvaluationError".into(),
                    value: &dead_lettering_on_filter_evaluation_error_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetteringOnMessageExpiration".into(),
                    value: &dead_lettering_on_message_expiration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultMessageTtl".into(),
                    value: &default_message_ttl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardDeadLetteredMessagesTo".into(),
                    value: &forward_dead_lettered_messages_to_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardTo".into(),
                    value: &forward_to_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockDuration".into(),
                    value: &lock_duration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxDeliveryCount".into(),
                    value: &max_delivery_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requiresSession".into(),
                    value: &requires_session_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicId".into(),
                    value: &topic_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriptionResult {
            auto_delete_on_idle: o.get_field("autoDeleteOnIdle"),
            batched_operations_enabled: o.get_field("batchedOperationsEnabled"),
            client_scoped_subscription: o.get_field("clientScopedSubscription"),
            client_scoped_subscription_enabled: o
                .get_field("clientScopedSubscriptionEnabled"),
            dead_lettering_on_filter_evaluation_error: o
                .get_field("deadLetteringOnFilterEvaluationError"),
            dead_lettering_on_message_expiration: o
                .get_field("deadLetteringOnMessageExpiration"),
            default_message_ttl: o.get_field("defaultMessageTtl"),
            forward_dead_lettered_messages_to: o
                .get_field("forwardDeadLetteredMessagesTo"),
            forward_to: o.get_field("forwardTo"),
            lock_duration: o.get_field("lockDuration"),
            max_delivery_count: o.get_field("maxDeliveryCount"),
            name: o.get_field("name"),
            requires_session: o.get_field("requiresSession"),
            status: o.get_field("status"),
            topic_id: o.get_field("topicId"),
        }
    }
}
