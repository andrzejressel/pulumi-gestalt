/// Manages a Digital Twins Event Hub Endpoint.
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
///             .name("example_resources")
///             .build_struct(),
///     );
///     let exampleAuthorizationRule = authorization_rule::create(
///         "exampleAuthorizationRule",
///         AuthorizationRuleArgs::builder()
///             .eventhub_name("${exampleEventHub.name}")
///             .listen(false)
///             .manage(false)
///             .name("example-ar")
///             .namespace_name("${exampleEventHubNamespace.name}")
///             .resource_group_name("${example.name}")
///             .send(true)
///             .build_struct(),
///     );
///     let exampleEndpointEventHub = endpoint_event_hub::create(
///         "exampleEndpointEventHub",
///         EndpointEventHubArgs::builder()
///             .digital_twins_id("${exampleInstance.id}")
///             .eventhub_primary_connection_string(
///                 "${exampleAuthorizationRule.primaryConnectionString}",
///             )
///             .eventhub_secondary_connection_string(
///                 "${exampleAuthorizationRule.secondaryConnectionString}",
///             )
///             .name("example-EH")
///             .build_struct(),
///     );
///     let exampleEventHub = event_hub::create(
///         "exampleEventHub",
///         EventHubArgs::builder()
///             .message_retention(1)
///             .name("example-eh")
///             .namespace_name("${exampleEventHubNamespace.name}")
///             .partition_count(2)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEventHubNamespace = event_hub_namespace::create(
///         "exampleEventHubNamespace",
///         EventHubNamespaceArgs::builder()
///             .location("${example.location}")
///             .name("example-eh-ns")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleInstance = instance::create(
///         "exampleInstance",
///         InstanceArgs::builder()
///             .location("${example.location}")
///             .name("example-DT")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Digital Twins Eventhub Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:digitaltwins/endpointEventHub:EndpointEventHub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DigitalTwins/digitalTwinsInstances/dt1/endpoints/ep1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_event_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointEventHubArgs {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        #[builder(into, default)]
        pub dead_letter_storage_secret: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The resource ID of the Digital Twins Instance. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        #[builder(into)]
        pub digital_twins_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The primary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        #[builder(into)]
        pub eventhub_primary_connection_string: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The secondary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        #[builder(into)]
        pub eventhub_secondary_connection_string: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name which should be used for this Digital Twins Event Hub Endpoint. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EndpointEventHubResult {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        pub dead_letter_storage_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the Digital Twins Instance. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        pub digital_twins_id: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        pub eventhub_primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        pub eventhub_secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Digital Twins Event Hub Endpoint. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointEventHubArgs,
    ) -> EndpointEventHubResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dead_letter_storage_secret_binding = args
            .dead_letter_storage_secret
            .get_output(context);
        let digital_twins_id_binding = args.digital_twins_id.get_output(context);
        let eventhub_primary_connection_string_binding = args
            .eventhub_primary_connection_string
            .get_output(context);
        let eventhub_secondary_connection_string_binding = args
            .eventhub_secondary_connection_string
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:digitaltwins/endpointEventHub:EndpointEventHub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterStorageSecret".into(),
                    value: &dead_letter_storage_secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "digitalTwinsId".into(),
                    value: &digital_twins_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubPrimaryConnectionString".into(),
                    value: &eventhub_primary_connection_string_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubSecondaryConnectionString".into(),
                    value: &eventhub_secondary_connection_string_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointEventHubResult {
            dead_letter_storage_secret: o.get_field("deadLetterStorageSecret"),
            digital_twins_id: o.get_field("digitalTwinsId"),
            eventhub_primary_connection_string: o
                .get_field("eventhubPrimaryConnectionString"),
            eventhub_secondary_connection_string: o
                .get_field("eventhubSecondaryConnectionString"),
            name: o.get_field("name"),
        }
    }
}
