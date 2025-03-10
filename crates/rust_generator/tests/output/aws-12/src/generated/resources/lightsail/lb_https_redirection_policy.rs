/// Configures Https Redirection for a Lightsail Load Balancer. A valid Certificate must be attached to the load balancer in order to enable https redirection.
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
///   testLbHttpsRedirectionPolicy:
///     type: aws:lightsail:LbHttpsRedirectionPolicy
///     name: test
///     properties:
///       lbName: ${test.name}
///       enabled: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_https_redirection_policy` using the `lb_name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbHttpsRedirectionPolicy:LbHttpsRedirectionPolicy test example-load-balancer
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lb_https_redirection_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbHttpsRedirectionPolicyArgs {
        /// The Https Redirection state of the load balancer. `true` to activate http to https redirection or `false` to deactivate http to https redirection.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The name of the load balancer to which you want to enable http to https redirection.
        #[builder(into)]
        pub lb_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LbHttpsRedirectionPolicyResult {
        /// The Https Redirection state of the load balancer. `true` to activate http to https redirection or `false` to deactivate http to https redirection.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name of the load balancer to which you want to enable http to https redirection.
        pub lb_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbHttpsRedirectionPolicyArgs,
    ) -> LbHttpsRedirectionPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let lb_name_binding = args.lb_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/lbHttpsRedirectionPolicy:LbHttpsRedirectionPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LbHttpsRedirectionPolicyResult {
            enabled: o.get_field("enabled"),
            lb_name: o.get_field("lbName"),
        }
    }
}
