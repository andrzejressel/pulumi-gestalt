/// Manages an Email Communication Service Domain.
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
///     let exampleEmailService = email_service::create(
///         "exampleEmailService",
///         EmailServiceArgs::builder()
///             .data_location("United States")
///             .name("example-emailcommunicationservice")
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
/// }
/// ```
///
/// ## Import
///
/// Communication Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:communication/emailServiceDomain:EmailServiceDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Communication/emailServices/emailCommunicationService1/domains/domain1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_service_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailServiceDomainArgs {
        /// Describes how a Domains resource is being managed. Possible values are `AzureManaged`, `CustomerManaged`, `CustomerManagedInExchangeOnline`. Changing this forces a new Email Communication Service to be created.
        #[builder(into)]
        pub domain_management: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Email Communication Service where the Domain belongs to. Changing this forces a new Email Communication Service to be created.
        #[builder(into)]
        pub email_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Email Communication Service resource. If `domain_management` is `AzureManaged`, the name must be `AzureManagedDomain`. Changing this forces a new Email Communication Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Email Communication Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Describes user engagement tracking is enabled or disabled. Defaults to `false`.
        #[builder(into, default)]
        pub user_engagement_tracking_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct EmailServiceDomainResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Describes how a Domains resource is being managed. Possible values are `AzureManaged`, `CustomerManaged`, `CustomerManagedInExchangeOnline`. Changing this forces a new Email Communication Service to be created.
        pub domain_management: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Email Communication Service where the Domain belongs to. Changing this forces a new Email Communication Service to be created.
        pub email_service_id: pulumi_gestalt_rust::Output<String>,
        /// P2 sender domain that is displayed to the email recipients [RFC 5322].
        pub from_sender_domain: pulumi_gestalt_rust::Output<String>,
        /// P1 sender domain that is present on the email envelope [RFC 5321].
        pub mail_from_sender_domain: pulumi_gestalt_rust::Output<String>,
        /// The name of the Email Communication Service resource. If `domain_management` is `AzureManaged`, the name must be `AzureManagedDomain`. Changing this forces a new Email Communication Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Email Communication Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Describes user engagement tracking is enabled or disabled. Defaults to `false`.
        pub user_engagement_tracking_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// (Optional) An `verification_records` block as defined below.
        pub verification_records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::communication::EmailServiceDomainVerificationRecord>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailServiceDomainArgs,
    ) -> EmailServiceDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_management_binding = args.domain_management.get_output(context);
        let email_service_id_binding = args.email_service_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_engagement_tracking_enabled_binding = args
            .user_engagement_tracking_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:communication/emailServiceDomain:EmailServiceDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainManagement".into(),
                    value: &domain_management_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailServiceId".into(),
                    value: &email_service_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userEngagementTrackingEnabled".into(),
                    value: &user_engagement_tracking_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailServiceDomainResult {
            id: o.get_field("id"),
            domain_management: o.get_field("domainManagement"),
            email_service_id: o.get_field("emailServiceId"),
            from_sender_domain: o.get_field("fromSenderDomain"),
            mail_from_sender_domain: o.get_field("mailFromSenderDomain"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            user_engagement_tracking_enabled: o
                .get_field("userEngagementTrackingEnabled"),
            verification_records: o.get_field("verificationRecords"),
        }
    }
}
