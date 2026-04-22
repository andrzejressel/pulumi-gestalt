/// Manages a Active Directory Domain Service Trust.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleServiceTrust:
///     type: azure:domainservices:ServiceTrust
///     name: example
///     properties:
///       name: example-trust
///       domainServiceId: ${example.id}
///       trustedDomainFqdn: example.com
///       trustedDomainDnsIps:
///         - 10.1.0.3
///         - 10.1.0.4
///       password: Password123
/// variables:
///   example:
///     fn::invoke:
///       function: azure:domainservices:getService
///       arguments:
///         name: example-ds
///         resourceGroupName: example-rg
/// ```
///
/// ## Import
///
/// Active Directory Domain Service Trusts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:domainservices/serviceTrust:ServiceTrust example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.AAD/domainServices/DomainService1/trusts/trust1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod service_trust {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceTrustArgs {
        /// The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
        #[builder(into)]
        pub domain_service_id: pulumi_gestalt_rust::Input<String>,
        /// The name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The password of the inbound trust set in the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::Input<String>,
        /// Specifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub trusted_domain_dns_ips: pulumi_gestalt_rust::Input<Vec<String>>,
        /// The FQDN of the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub trusted_domain_fqdn: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceTrustResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
        pub domain_service_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password of the inbound trust set in the on-premise Active Directory Domain Service.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
        pub trusted_domain_dns_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The FQDN of the on-premise Active Directory Domain Service.
        pub trusted_domain_fqdn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceTrustArgs,
    ) -> ServiceTrustResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceTrustArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ServiceTrustResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceTrustArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ServiceTrustResult {
        let domain_service_id_binding = args.domain_service_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let password_binding = args.password.get_output(ctx);
        let trusted_domain_dns_ips_binding = args.trusted_domain_dns_ips.get_output(ctx);
        let trusted_domain_fqdn_binding = args.trusted_domain_fqdn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:domainservices/serviceTrust:ServiceTrust".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainServiceId".into(),
                    value: &domain_service_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedDomainDnsIps".into(),
                    value: &trusted_domain_dns_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedDomainFqdn".into(),
                    value: &trusted_domain_fqdn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ServiceTrustResult {
            id: o.get_id(),
            urn: o.get_urn(),
            domain_service_id: o.get_field("domainServiceId"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            trusted_domain_dns_ips: o.get_field("trustedDomainDnsIps"),
            trusted_domain_fqdn: o.get_field("trustedDomainFqdn"),
        }
    }
}
