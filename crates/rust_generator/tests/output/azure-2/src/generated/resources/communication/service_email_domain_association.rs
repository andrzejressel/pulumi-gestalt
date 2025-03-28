/// Manages a communication service email domain association.
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
///             .name("group1")
///             .build_struct(),
///     );
///     let exampleEmailService = email_service::create(
///         "exampleEmailService",
///         EmailServiceArgs::builder()
///             .data_location("United States")
///             .name("emailCommunicationService1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEmailServiceDomain = email_service_domain::create(
///         "exampleEmailServiceDomain",
///         EmailServiceDomainArgs::builder()
///             .domain_management("AzureManaged")
///             .email_service_id("${exampleEmailService.id}")
///             .name("AzureManagedDomain")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .data_location("United States")
///             .name("CommunicationService1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleServiceEmailDomainAssociation = service_email_domain_association::create(
///         "exampleServiceEmailDomainAssociation",
///         ServiceEmailDomainAssociationArgs::builder()
///             .communication_service_id("${exampleService.id}")
///             .email_service_domain_id("${exampleEmailServiceDomain.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Communication service email domain association can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:communication/serviceEmailDomainAssociation:ServiceEmailDomainAssociation example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Communication/communicationServices/communicationService1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Communication/emailServices/emailCommunicationService1/domains/domain1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_email_domain_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceEmailDomainAssociationArgs {
        /// The ID of the Communication Service. Changing this forces a new communication service email domain association to be created.
        #[builder(into)]
        pub communication_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the EMail Service Domain. Changing this forces a new communication service email domain association to be created.
        #[builder(into)]
        pub email_service_domain_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceEmailDomainAssociationResult {
        /// The ID of the Communication Service. Changing this forces a new communication service email domain association to be created.
        pub communication_service_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the EMail Service Domain. Changing this forces a new communication service email domain association to be created.
        pub email_service_domain_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceEmailDomainAssociationArgs,
    ) -> ServiceEmailDomainAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let communication_service_id_binding = args
            .communication_service_id
            .get_output(context);
        let email_service_domain_id_binding = args
            .email_service_domain_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:communication/serviceEmailDomainAssociation:ServiceEmailDomainAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "communicationServiceId".into(),
                    value: &communication_service_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailServiceDomainId".into(),
                    value: &email_service_domain_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceEmailDomainAssociationResult {
            communication_service_id: o.get_field("communicationServiceId"),
            email_service_domain_id: o.get_field("emailServiceDomainId"),
        }
    }
}
