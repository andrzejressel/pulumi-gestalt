/// Attaches a Lightsail Load Balancer Certificate to a Lightsail Load Balancer.
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
///   testLbCertificateAttachment:
///     type: aws:lightsail:LbCertificateAttachment
///     name: test
///     properties:
///       lbName: ${test.name}
///       certificateName: ${testLbCertificate.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_certificate_attachment` using the name attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbCertificateAttachment:LbCertificateAttachment test example-load-balancer,example-certificate
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod lb_certificate_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbCertificateAttachmentArgs {
        /// The name of your SSL/TLS certificate.
        #[builder(into)]
        pub certificate_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the load balancer to which you want to associate the SSL/TLS certificate.
        #[builder(into)]
        pub lb_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct LbCertificateAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of your SSL/TLS certificate.
        pub certificate_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the load balancer to which you want to associate the SSL/TLS certificate.
        pub lb_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbCertificateAttachmentArgs,
    ) -> LbCertificateAttachmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbCertificateAttachmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LbCertificateAttachmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbCertificateAttachmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LbCertificateAttachmentResult {
        let certificate_name_binding = args.certificate_name.get_output(ctx);
        let lb_name_binding = args.lb_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/lbCertificateAttachment:LbCertificateAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LbCertificateAttachmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            certificate_name: o.get_field("certificateName"),
            lb_name: o.get_field("lbName"),
        }
    }
}
