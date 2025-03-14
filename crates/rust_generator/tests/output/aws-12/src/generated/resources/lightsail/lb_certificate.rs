/// Creates a Lightsail load balancer Certificate resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Lb
///     properties:
///       name: test-load-balancer
///       healthCheckPath: /
///       instancePort: '80'
///       tags:
///         foo: bar
///   testLbCertificate:
///     type: aws:lightsail:LbCertificate
///     name: test
///     properties:
///       name: test-load-balancer-certificate
///       lbName: ${test.id}
///       domainName: test.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_certificate` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbCertificate:LbCertificate test example-load-balancer,example-load-balancer-certificate
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lb_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbCertificateArgs {
        /// The domain name (e.g., example.com) for your SSL/TLS certificate.
        #[builder(into, default)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The load balancer name where you want to create the SSL/TLS certificate.
        #[builder(into)]
        pub lb_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SSL/TLS certificate name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        #[builder(into, default)]
        pub subject_alternative_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LbCertificateResult {
        /// The ARN of the lightsail certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The domain name (e.g., example.com) for your SSL/TLS certificate.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        pub domain_validation_records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::lightsail::LbCertificateDomainValidationRecord>,
        >,
        /// The load balancer name where you want to create the SSL/TLS certificate.
        pub lb_name: pulumi_gestalt_rust::Output<String>,
        /// The SSL/TLS certificate name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        pub subject_alternative_names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub support_code: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbCertificateArgs,
    ) -> LbCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let lb_name_binding = args.lb_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let subject_alternative_names_binding = args
            .subject_alternative_names
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/lbCertificate:LbCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subjectAlternativeNames".into(),
                    value: &subject_alternative_names_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LbCertificateResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            domain_name: o.get_field("domainName"),
            domain_validation_records: o.get_field("domainValidationRecords"),
            lb_name: o.get_field("lbName"),
            name: o.get_field("name"),
            subject_alternative_names: o.get_field("subjectAlternativeNames"),
            support_code: o.get_field("supportCode"),
        }
    }
}
