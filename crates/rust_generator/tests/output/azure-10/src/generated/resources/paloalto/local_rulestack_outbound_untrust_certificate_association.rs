/// Manages a Palo Alto Networks Rulestack Outbound Untrust Certificate Association.
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
///             .name("rg-example")
///             .build_struct(),
///     );
///     let exampleLocalRulestack = local_rulestack::create(
///         "exampleLocalRulestack",
///         LocalRulestackArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLocalRulestackCertificate = local_rulestack_certificate::create(
///         "exampleLocalRulestackCertificate",
///         LocalRulestackCertificateArgs::builder()
///             .name("example")
///             .rulestack_id("${exampleLocalRulestack.id}")
///             .self_signed(true)
///             .build_struct(),
///     );
///     let exampleLocalRulestackOutboundUntrustCertificateAssociation = local_rulestack_outbound_untrust_certificate_association::create(
///         "exampleLocalRulestackOutboundUntrustCertificateAssociation",
///         LocalRulestackOutboundUntrustCertificateAssociationArgs::builder()
///             .certificate_id("${exampleLocalRulestackCertificate.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_rulestack_outbound_untrust_certificate_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackOutboundUntrustCertificateAssociationArgs {
        /// The ID of the Certificate to use as the Outbound Untrust Certificate. Changing this forces a new Palo Alto Networks Rulestack Outbound Untrust Certificate Association to be created.
        #[builder(into)]
        pub certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackOutboundUntrustCertificateAssociationResult {
        /// The ID of the Certificate to use as the Outbound Untrust Certificate. Changing this forces a new Palo Alto Networks Rulestack Outbound Untrust Certificate Association to be created.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalRulestackOutboundUntrustCertificateAssociationArgs,
    ) -> LocalRulestackOutboundUntrustCertificateAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_id_binding = args.certificate_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackOutboundUntrustCertificateAssociation:LocalRulestackOutboundUntrustCertificateAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalRulestackOutboundUntrustCertificateAssociationResult {
            certificate_id: o.get_field("certificateId"),
        }
    }
}
