/// Provides a resource to manage a domain that has been [registered](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/registrar-tld-list.html) and associated with the current AWS account.
///
/// **This is an advanced resource** and has special caveats to be aware of when using it. Please read this document in its entirety before using this resource.
///
/// The `aws.route53domains.RegisteredDomain` resource behaves differently from normal resources in that if a domain has been registered, the provider does not _register_ this domain, but instead "adopts" it into management. A destroy does not delete the domain but does remove the resource from state.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53domains:RegisteredDomain
///     properties:
///       domainName: example.com
///       nameServers:
///         - name: ns-195.awsdns-24.com
///         - name: ns-874.awsdns-45.net
///       tags:
///         Environment: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import domains using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:route53domains/registeredDomain:RegisteredDomain example example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registered_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegisteredDomainArgs {
        /// Details about the domain administrative contact. See Contact Blocks for more details.
        #[builder(into, default)]
        pub admin_contact: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::route53domains::RegisteredDomainAdminContact>,
        >,
        /// Whether domain administrative contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub admin_privacy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the domain registration is set to renew automatically. Default: `true`.
        #[builder(into, default)]
        pub auto_renew: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Details about the domain billing contact. See Contact Blocks for more details.
        #[builder(into, default)]
        pub billing_contact: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::route53domains::RegisteredDomainBillingContact>,
        >,
        /// Whether domain billing contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub billing_privacy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the registered domain.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of nameservers for the domain. See `name_server` Blocks for more details.
        #[builder(into, default)]
        pub name_servers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::route53domains::RegisteredDomainNameServer>>,
        >,
        /// Details about the domain registrant. See Contact Blocks for more details.
        #[builder(into, default)]
        pub registrant_contact: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::route53domains::RegisteredDomainRegistrantContact,
            >,
        >,
        /// Whether domain registrant contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub registrant_privacy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Details about the domain technical contact. See Contact Blocks for more details.
        #[builder(into, default)]
        pub tech_contact: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::route53domains::RegisteredDomainTechContact>,
        >,
        /// Whether domain technical contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub tech_privacy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the domain is locked for transfer. Default: `true`.
        #[builder(into, default)]
        pub transfer_lock: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RegisteredDomainResult {
        /// Email address to contact to report incorrect contact information for a domain, to report that the domain is being used to send spam, to report that someone is cybersquatting on a domain name, or report some other type of abuse.
        pub abuse_contact_email: pulumi_gestalt_rust::Output<String>,
        /// Phone number for reporting abuse.
        pub abuse_contact_phone: pulumi_gestalt_rust::Output<String>,
        /// Details about the domain administrative contact. See Contact Blocks for more details.
        pub admin_contact: pulumi_gestalt_rust::Output<
            super::super::types::route53domains::RegisteredDomainAdminContact,
        >,
        /// Whether domain administrative contact information is concealed from WHOIS queries. Default: `true`.
        pub admin_privacy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the domain registration is set to renew automatically. Default: `true`.
        pub auto_renew: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Details about the domain billing contact. See Contact Blocks for more details.
        pub billing_contact: pulumi_gestalt_rust::Output<
            super::super::types::route53domains::RegisteredDomainBillingContact,
        >,
        /// Whether domain billing contact information is concealed from WHOIS queries. Default: `true`.
        pub billing_privacy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The date when the domain was created as found in the response to a WHOIS query.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// The name of the registered domain.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The date when the registration for the domain is set to expire.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The list of nameservers for the domain. See `name_server` Blocks for more details.
        pub name_servers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::route53domains::RegisteredDomainNameServer>,
        >,
        /// Details about the domain registrant. See Contact Blocks for more details.
        pub registrant_contact: pulumi_gestalt_rust::Output<
            super::super::types::route53domains::RegisteredDomainRegistrantContact,
        >,
        /// Whether domain registrant contact information is concealed from WHOIS queries. Default: `true`.
        pub registrant_privacy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the registrar of the domain as identified in the registry.
        pub registrar_name: pulumi_gestalt_rust::Output<String>,
        /// Web address of the registrar.
        pub registrar_url: pulumi_gestalt_rust::Output<String>,
        /// Reseller of the domain.
        pub reseller: pulumi_gestalt_rust::Output<String>,
        /// List of [domain name status codes](https://www.icann.org/resources/pages/epp-status-codes-2014-06-16-en).
        pub status_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Details about the domain technical contact. See Contact Blocks for more details.
        pub tech_contact: pulumi_gestalt_rust::Output<
            super::super::types::route53domains::RegisteredDomainTechContact,
        >,
        /// Whether domain technical contact information is concealed from WHOIS queries. Default: `true`.
        pub tech_privacy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the domain is locked for transfer. Default: `true`.
        pub transfer_lock: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The last updated date of the domain as found in the response to a WHOIS query.
        pub updated_date: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified name of the WHOIS server that can answer the WHOIS query for the domain.
        pub whois_server: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegisteredDomainArgs,
    ) -> RegisteredDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_contact_binding = args.admin_contact.get_output(context);
        let admin_privacy_binding = args.admin_privacy.get_output(context);
        let auto_renew_binding = args.auto_renew.get_output(context);
        let billing_contact_binding = args.billing_contact.get_output(context);
        let billing_privacy_binding = args.billing_privacy.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let name_servers_binding = args.name_servers.get_output(context);
        let registrant_contact_binding = args.registrant_contact.get_output(context);
        let registrant_privacy_binding = args.registrant_privacy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tech_contact_binding = args.tech_contact.get_output(context);
        let tech_privacy_binding = args.tech_privacy.get_output(context);
        let transfer_lock_binding = args.transfer_lock.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53domains/registeredDomain:RegisteredDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminContact".into(),
                    value: &admin_contact_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminPrivacy".into(),
                    value: &admin_privacy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoRenew".into(),
                    value: &auto_renew_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingContact".into(),
                    value: &billing_contact_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingPrivacy".into(),
                    value: &billing_privacy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nameServers".into(),
                    value: &name_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registrantContact".into(),
                    value: &registrant_contact_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registrantPrivacy".into(),
                    value: &registrant_privacy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "techContact".into(),
                    value: &tech_contact_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "techPrivacy".into(),
                    value: &tech_privacy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transferLock".into(),
                    value: &transfer_lock_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegisteredDomainResult {
            abuse_contact_email: o.get_field("abuseContactEmail"),
            abuse_contact_phone: o.get_field("abuseContactPhone"),
            admin_contact: o.get_field("adminContact"),
            admin_privacy: o.get_field("adminPrivacy"),
            auto_renew: o.get_field("autoRenew"),
            billing_contact: o.get_field("billingContact"),
            billing_privacy: o.get_field("billingPrivacy"),
            creation_date: o.get_field("creationDate"),
            domain_name: o.get_field("domainName"),
            expiration_date: o.get_field("expirationDate"),
            name_servers: o.get_field("nameServers"),
            registrant_contact: o.get_field("registrantContact"),
            registrant_privacy: o.get_field("registrantPrivacy"),
            registrar_name: o.get_field("registrarName"),
            registrar_url: o.get_field("registrarUrl"),
            reseller: o.get_field("reseller"),
            status_lists: o.get_field("statusLists"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tech_contact: o.get_field("techContact"),
            tech_privacy: o.get_field("techPrivacy"),
            transfer_lock: o.get_field("transferLock"),
            updated_date: o.get_field("updatedDate"),
            whois_server: o.get_field("whoisServer"),
        }
    }
}
