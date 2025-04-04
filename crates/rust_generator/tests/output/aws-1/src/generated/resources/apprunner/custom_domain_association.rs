/// Manages an App Runner Custom Domain association.
///
/// > **NOTE:** After creation, you must use the information in the `certification_validation_records` attribute to add CNAME records to your Domain Name System (DNS). For each mapped domain name, add a mapping to the target App Runner subdomain (found in the `dns_target` attribute) and one or more certificate validation records. App Runner then performs DNS validation to verify that you own or control the domain name you associated. App Runner tracks domain validity in a certificate stored in AWS Certificate Manager (ACM).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_domain_association::create(
///         "example",
///         CustomDomainAssociationArgs::builder()
///             .domain_name("example.com")
///             .service_arn("${exampleAwsApprunnerService.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner Custom Domain Associations using the `domain_name` and `service_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/customDomainAssociation:CustomDomainAssociation example example.com,arn:aws:apprunner:us-east-1:123456789012:service/example-app/8fe1e10304f84fd2b0df550fe98a71fa
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_domain_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDomainAssociationArgs {
        /// Custom domain endpoint to association. Specify a base domain e.g., `example.com` or a subdomain e.g., `subdomain.example.com`.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to associate the subdomain with the App Runner service in addition to the base domain. Defaults to `true`.
        #[builder(into, default)]
        pub enable_www_subdomain: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ARN of the App Runner service.
        #[builder(into)]
        pub service_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomDomainAssociationResult {
        /// A set of certificate CNAME records used for this domain name. See Certificate Validation Records below for more details.
        pub certificate_validation_records: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::apprunner::CustomDomainAssociationCertificateValidationRecord,
            >,
        >,
        /// App Runner subdomain of the App Runner service. The custom domain name is mapped to this target name. Attribute only available if resource created (not imported) with this provider.
        pub dns_target: pulumi_gestalt_rust::Output<String>,
        /// Custom domain endpoint to association. Specify a base domain e.g., `example.com` or a subdomain e.g., `subdomain.example.com`.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Whether to associate the subdomain with the App Runner service in addition to the base domain. Defaults to `true`.
        pub enable_www_subdomain: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN of the App Runner service.
        pub service_arn: pulumi_gestalt_rust::Output<String>,
        /// Current state of the certificate CNAME record validation. It should change to `SUCCESS` after App Runner completes validation with your DNS.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomDomainAssociationArgs,
    ) -> CustomDomainAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let enable_www_subdomain_binding = args.enable_www_subdomain.get_output(context);
        let service_arn_binding = args.service_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/customDomainAssociation:CustomDomainAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableWwwSubdomain".into(),
                    value: &enable_www_subdomain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceArn".into(),
                    value: &service_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomDomainAssociationResult {
            certificate_validation_records: o.get_field("certificateValidationRecords"),
            dns_target: o.get_field("dnsTarget"),
            domain_name: o.get_field("domainName"),
            enable_www_subdomain: o.get_field("enableWwwSubdomain"),
            service_arn: o.get_field("serviceArn"),
            status: o.get_field("status"),
        }
    }
}
