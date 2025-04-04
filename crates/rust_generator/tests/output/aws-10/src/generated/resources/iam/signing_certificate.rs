/// Provides an IAM Signing Certificate resource to upload Signing Certificates.
///
/// > **Note:** All arguments including the certificate body will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// **Using certs on file:**
///
/// ```yaml
/// resources:
///   testCert:
///     type: aws:iam:SigningCertificate
///     name: test_cert
///     properties:
///       username: some_test_cert
///       certificateBody:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: self-ca-cert.pem
///           return: result
/// ```
///
/// **Example with cert in-line:**
///
/// ```yaml
/// resources:
///   testCertAlt:
///     type: aws:iam:SigningCertificate
///     name: test_cert_alt
///     properties:
///       username: some_test_cert
///       certificateBody: |
///         -----BEGIN CERTIFICATE-----
///         [......] # cert contents
///         -----END CERTIFICATE-----
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Signing Certificates using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/signingCertificate:SigningCertificate certificate IDIDIDIDID:user-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod signing_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SigningCertificateArgs {
        /// The contents of the signing certificate in PEM-encoded format.
        #[builder(into)]
        pub certificate_body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The status you want to assign to the certificate. `Active` means that the certificate can be used for programmatic calls to Amazon Web Services `Inactive` means that the certificate cannot be used.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the user the signing certificate is for.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SigningCertificateResult {
        /// The contents of the signing certificate in PEM-encoded format.
        pub certificate_body: pulumi_gestalt_rust::Output<String>,
        /// The ID for the signing certificate.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        /// The status you want to assign to the certificate. `Active` means that the certificate can be used for programmatic calls to Amazon Web Services `Inactive` means that the certificate cannot be used.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the user the signing certificate is for.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SigningCertificateArgs,
    ) -> SigningCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_body_binding = args.certificate_body.get_output(context);
        let status_binding = args.status.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/signingCertificate:SigningCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateBody".into(),
                    value: &certificate_body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SigningCertificateResult {
            certificate_body: o.get_field("certificateBody"),
            certificate_id: o.get_field("certificateId"),
            status: o.get_field("status"),
            user_name: o.get_field("userName"),
        }
    }
}
