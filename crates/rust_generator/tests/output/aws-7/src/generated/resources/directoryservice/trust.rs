/// Manages a trust relationship between two Active Directory Directories.
///
/// The directories may either be both AWS Managed Microsoft AD domains or an AWS Managed Microsoft AD domain and a self-managed Active Directory Domain.
///
/// The Trust relationship must be configured on both sides of the relationship.
/// If a Trust has only been created on one side, it will be in the state `VerifyFailed`.
/// Once the second Trust is created, the first will update to the correct state.
///
/// ## Example Usage
///
/// ### Two-Way Trust
///
/// ```yaml
/// resources:
///   one:
///     type: aws:directoryservice:Trust
///     properties:
///       directoryId: ${oneDirectory.id}
///       remoteDomainName: ${twoDirectory.name}
///       trustDirection: Two-Way
///       trustPassword: Some0therPassword
///       conditionalForwarderIpAddrs: ${twoDirectory.dnsIpAddresses}
///   two:
///     type: aws:directoryservice:Trust
///     properties:
///       directoryId: ${twoDirectory.id}
///       remoteDomainName: ${oneDirectory.name}
///       trustDirection: Two-Way
///       trustPassword: Some0therPassword
///       conditionalForwarderIpAddrs: ${oneDirectory.dnsIpAddresses}
///   oneDirectory:
///     type: aws:directoryservice:Directory
///     name: one
///     properties:
///       name: one.example.com
///       type: MicrosoftAD
///   twoDirectory:
///     type: aws:directoryservice:Directory
///     name: two
///     properties:
///       name: two.example.com
///       type: MicrosoftAD
/// ```
///
/// ### One-Way Trust
///
/// ```yaml
/// resources:
///   one:
///     type: aws:directoryservice:Trust
///     properties:
///       directoryId: ${oneDirectory.id}
///       remoteDomainName: ${twoDirectory.name}
///       trustDirection: 'One-Way: Incoming'
///       trustPassword: Some0therPassword
///       conditionalForwarderIpAddrs: ${twoDirectory.dnsIpAddresses}
///   two:
///     type: aws:directoryservice:Trust
///     properties:
///       directoryId: ${twoDirectory.id}
///       remoteDomainName: ${oneDirectory.name}
///       trustDirection: 'One-Way: Outgoing'
///       trustPassword: Some0therPassword
///       conditionalForwarderIpAddrs: ${oneDirectory.dnsIpAddresses}
///   oneDirectory:
///     type: aws:directoryservice:Directory
///     name: one
///     properties:
///       name: one.example.com
///       type: MicrosoftAD
///   twoDirectory:
///     type: aws:directoryservice:Directory
///     name: two
///     properties:
///       name: two.example.com
///       type: MicrosoftAD
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the Trust relationship using the directory ID and remote domain name, separated by a `/`. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/trust:Trust example d-926724cf57/directory.example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trust {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustArgs {
        /// Set of IPv4 addresses for the DNS server associated with the remote Directory.
        /// Can contain between 1 and 4 values.
        #[builder(into, default)]
        pub conditional_forwarder_ip_addrs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether to delete the conditional forwarder when deleting the Trust relationship.
        #[builder(into, default)]
        pub delete_associated_conditional_forwarder: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ID of the Directory.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Fully qualified domain name of the remote Directory.
        #[builder(into)]
        pub remote_domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable selective authentication.
        /// Valid values are `Enabled` and `Disabled`.
        /// Default value is `Disabled`.
        #[builder(into, default)]
        pub selective_auth: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The direction of the Trust relationship.
        /// Valid values are `One-Way: Outgoing`, `One-Way: Incoming`, and `Two-Way`.
        #[builder(into)]
        pub trust_direction: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Password for the Trust.
        /// Does not need to match the passwords for either Directory.
        /// Can contain upper- and lower-case letters, numbers, and punctuation characters.
        /// May be up to 128 characters long.
        #[builder(into)]
        pub trust_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the Trust relationship.
        /// Valid values are `Forest` and `External`.
        /// Default value is `Forest`.
        #[builder(into, default)]
        pub trust_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TrustResult {
        /// Set of IPv4 addresses for the DNS server associated with the remote Directory.
        /// Can contain between 1 and 4 values.
        pub conditional_forwarder_ip_addrs: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Date and time when the Trust was created.
        pub created_date_time: pulumi_gestalt_rust::Output<String>,
        /// Whether to delete the conditional forwarder when deleting the Trust relationship.
        pub delete_associated_conditional_forwarder: pulumi_gestalt_rust::Output<bool>,
        /// ID of the Directory.
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the Trust was last updated.
        pub last_updated_date_time: pulumi_gestalt_rust::Output<String>,
        /// Fully qualified domain name of the remote Directory.
        pub remote_domain_name: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable selective authentication.
        /// Valid values are `Enabled` and `Disabled`.
        /// Default value is `Disabled`.
        pub selective_auth: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the Trust state in `trust_state` was last updated.
        pub state_last_updated_date_time: pulumi_gestalt_rust::Output<String>,
        /// The direction of the Trust relationship.
        /// Valid values are `One-Way: Outgoing`, `One-Way: Incoming`, and `Two-Way`.
        pub trust_direction: pulumi_gestalt_rust::Output<String>,
        /// Password for the Trust.
        /// Does not need to match the passwords for either Directory.
        /// Can contain upper- and lower-case letters, numbers, and punctuation characters.
        /// May be up to 128 characters long.
        pub trust_password: pulumi_gestalt_rust::Output<String>,
        /// State of the Trust relationship.
        /// One of `Created`, `VerifyFailed`,`Verified`, `UpdateFailed`,`Updated`,`Deleted`, or `Failed`.
        pub trust_state: pulumi_gestalt_rust::Output<String>,
        /// Reason for the Trust state set in `trust_state`.
        pub trust_state_reason: pulumi_gestalt_rust::Output<String>,
        /// Type of the Trust relationship.
        /// Valid values are `Forest` and `External`.
        /// Default value is `Forest`.
        pub trust_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustArgs,
    ) -> TrustResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let conditional_forwarder_ip_addrs_binding = args
            .conditional_forwarder_ip_addrs
            .get_output(context);
        let delete_associated_conditional_forwarder_binding = args
            .delete_associated_conditional_forwarder
            .get_output(context);
        let directory_id_binding = args.directory_id.get_output(context);
        let remote_domain_name_binding = args.remote_domain_name.get_output(context);
        let selective_auth_binding = args.selective_auth.get_output(context);
        let trust_direction_binding = args.trust_direction.get_output(context);
        let trust_password_binding = args.trust_password.get_output(context);
        let trust_type_binding = args.trust_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/trust:Trust".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditionalForwarderIpAddrs".into(),
                    value: &conditional_forwarder_ip_addrs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteAssociatedConditionalForwarder".into(),
                    value: &delete_associated_conditional_forwarder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteDomainName".into(),
                    value: &remote_domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selectiveAuth".into(),
                    value: &selective_auth_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustDirection".into(),
                    value: &trust_direction_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustPassword".into(),
                    value: &trust_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustType".into(),
                    value: &trust_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrustResult {
            conditional_forwarder_ip_addrs: o.get_field("conditionalForwarderIpAddrs"),
            created_date_time: o.get_field("createdDateTime"),
            delete_associated_conditional_forwarder: o
                .get_field("deleteAssociatedConditionalForwarder"),
            directory_id: o.get_field("directoryId"),
            last_updated_date_time: o.get_field("lastUpdatedDateTime"),
            remote_domain_name: o.get_field("remoteDomainName"),
            selective_auth: o.get_field("selectiveAuth"),
            state_last_updated_date_time: o.get_field("stateLastUpdatedDateTime"),
            trust_direction: o.get_field("trustDirection"),
            trust_password: o.get_field("trustPassword"),
            trust_state: o.get_field("trustState"),
            trust_state_reason: o.get_field("trustStateReason"),
            trust_type: o.get_field("trustType"),
        }
    }
}
