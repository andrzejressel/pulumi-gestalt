/// Manages a Healthcare Med Tech Service Fhir Destination.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleWorkspace:
///     type: azure:healthcare:Workspace
///     name: example
///     properties:
///       name: exampleworkspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-ehn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: example-eh
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 1
///       messageRetention: 1
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: $default
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///   exampleFhirService:
///     type: azure:healthcare:FhirService
///     name: example
///     properties:
///       name: examplefhir
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       workspaceId: ${exampleWorkspace.id}
///       kind: fhir-R4
///       authentication:
///         authority: https://login.microsoftonline.com/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
///         audience: https://examplefhir.fhir.azurehealthcareapis.com
///   exampleMedtechService:
///     type: azure:healthcare:MedtechService
///     name: example
///     properties:
///       name: examplemt
///       workspaceId: ${exampleWorkspace.id}
///       location: ${example.location}
///       eventhubNamespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       eventhubConsumerGroupName: ${exampleConsumerGroup.name}
///       deviceMappingJson:
///         fn::toJSON:
///           templateType: CollectionContent
///           template: []
///   exampleMedtechServiceFhirDestination:
///     type: azure:healthcare:MedtechServiceFhirDestination
///     name: example
///     properties:
///       name: examplemtdes
///       location: east us
///       medtechServiceId: ${exampleMedtechService.id}
///       destinationFhirServiceId: ${exampleFhirService.id}
///       destinationIdentityResolutionType: Create
///       destinationFhirMappingJson:
///         fn::toJSON:
///           templateType: CollectionFhirTemplate
///           template:
///             - templateType: CodeValueFhir
///               template:
///                 codes:
///                   - code: 8867-4
///                     system: http://loinc.org
///                     display: Heart rate
///                 periodInterval: 60
///                 typeName: heartrate
///                 value:
///                   defaultPeriod: 5000
///                   unit: count/min
///                   valueName: hr
///                   valueType: SampledData
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Healthcare Med Tech Service Fhir Destination can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/medtechServiceFhirDestination:MedtechServiceFhirDestination example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.HealthcareApis/workspaces/workspace1/iotConnectors/iotconnector1/fhirDestinations/destination1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod medtech_service_fhir_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MedtechServiceFhirDestinationArgs {
        /// Specifies the destination Fhir mappings of the Med Tech Service Fhir Destination.
        #[builder(into)]
        pub destination_fhir_mapping_json: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the destination fhir service id of the Med Tech Service Fhir Destination.
        #[builder(into)]
        pub destination_fhir_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the destination identity resolution type where the Healthcare Med Tech Service Fhir Destination should be created. Possible values are `Create`, `Lookup`.
        #[builder(into)]
        pub destination_identity_resolution_type: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Specifies the Azure Region where the Healthcare Med Tech Service Fhir Destination should be created. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Healthcare Med Tech Service where the Healthcare Med Tech Service Fhir Destination should exist. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        #[builder(into)]
        pub medtech_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Healthcare Med Tech Service Fhir Destination. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MedtechServiceFhirDestinationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the destination Fhir mappings of the Med Tech Service Fhir Destination.
        pub destination_fhir_mapping_json: pulumi_gestalt_rust::Output<String>,
        /// Specifies the destination fhir service id of the Med Tech Service Fhir Destination.
        pub destination_fhir_service_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the destination identity resolution type where the Healthcare Med Tech Service Fhir Destination should be created. Possible values are `Create`, `Lookup`.
        pub destination_identity_resolution_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Healthcare Med Tech Service Fhir Destination should be created. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Healthcare Med Tech Service where the Healthcare Med Tech Service Fhir Destination should exist. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        pub medtech_service_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Healthcare Med Tech Service Fhir Destination. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MedtechServiceFhirDestinationArgs,
    ) -> MedtechServiceFhirDestinationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_fhir_mapping_json_binding = args
            .destination_fhir_mapping_json
            .get_output(context);
        let destination_fhir_service_id_binding = args
            .destination_fhir_service_id
            .get_output(context);
        let destination_identity_resolution_type_binding = args
            .destination_identity_resolution_type
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let medtech_service_id_binding = args.medtech_service_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:healthcare/medtechServiceFhirDestination:MedtechServiceFhirDestination"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationFhirMappingJson".into(),
                    value: &destination_fhir_mapping_json_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationFhirServiceId".into(),
                    value: &destination_fhir_service_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationIdentityResolutionType".into(),
                    value: &destination_identity_resolution_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "medtechServiceId".into(),
                    value: &medtech_service_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MedtechServiceFhirDestinationResult {
            id: o.get_field("id"),
            destination_fhir_mapping_json: o.get_field("destinationFhirMappingJson"),
            destination_fhir_service_id: o.get_field("destinationFhirServiceId"),
            destination_identity_resolution_type: o
                .get_field("destinationIdentityResolutionType"),
            location: o.get_field("location"),
            medtech_service_id: o.get_field("medtechServiceId"),
            name: o.get_field("name"),
        }
    }
}
