#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_queue_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueAuthorizationRuleArgs {
        /// The name of this ServiceBus Queue Authorisation Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the ServiceBus Namespace.
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub queue_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the ServiceBus Queue.
        #[builder(into, default)]
        pub queue_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the ServiceBus Queue Authorisation Rule exists.
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQueueAuthorizationRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub listen: pulumi_gestalt_rust::Output<bool>,
        pub manage: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Primary Connection String for the ServiceBus Queue authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the ServiceBus Queue authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        pub queue_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub queue_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Secondary Connection String for the ServiceBus Queue authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the ServiceBus Queue authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        pub send: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQueueAuthorizationRuleArgs,
    ) -> GetQueueAuthorizationRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let queue_id_binding = args.queue_id.get_output(context);
        let queue_name_binding = args.queue_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:servicebus/getQueueAuthorizationRule:getQueueAuthorizationRule"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueId".into(),
                    value: &queue_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueName".into(),
                    value: &queue_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQueueAuthorizationRuleResult {
            id: o.get_field("id"),
            listen: o.get_field("listen"),
            manage: o.get_field("manage"),
            name: o.get_field("name"),
            namespace_name: o.get_field("namespaceName"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            primary_connection_string_alias: o.get_field("primaryConnectionStringAlias"),
            primary_key: o.get_field("primaryKey"),
            queue_id: o.get_field("queueId"),
            queue_name: o.get_field("queueName"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            secondary_connection_string_alias: o
                .get_field("secondaryConnectionStringAlias"),
            secondary_key: o.get_field("secondaryKey"),
            send: o.get_field("send"),
        }
    }
}
