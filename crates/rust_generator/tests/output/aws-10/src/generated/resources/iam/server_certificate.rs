/// Provides an IAM Server Certificate resource to upload Server Certificates.
/// Certs uploaded to IAM can easily work with other AWS services such as:
///
/// - AWS Elastic Beanstalk
/// - Elastic Load Balancing
/// - CloudFront
/// - AWS OpsWorks
///
/// For information about server certificates in IAM, see [Managing Server
/// Certificates][2] in AWS Documentation.
///
/// ## Example Usage
///
/// **Using certs on file:**
///
/// ```yaml
/// resources:
///   testCert:
///     type: aws:iam:ServerCertificate
///     name: test_cert
///     properties:
///       name: some_test_cert
///       certificateBody:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: self-ca-cert.pem
///           return: result
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: test-key.pem
///           return: result
/// ```
///
/// **Example with cert in-line:**
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testCertAlt = server_certificate::create(
///         "testCertAlt",
///         ServerCertificateArgs::builder()
///             .certificate_body(
///                 "-----BEGIN CERTIFICATE-----\n[......] # cert contents\n-----END CERTIFICATE-----\n",
///             )
///             .name("alt_test_cert")
///             .private_key(
///                 "-----BEGIN RSA PRIVATE KEY-----\n[......] # cert contents\n-----END RSA PRIVATE KEY-----",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// **Use in combination with an AWS ELB resource:**
///
/// Some properties of an IAM Server Certificates cannot be updated while they are
/// in use. In order for the provider to effectively manage a Certificate in this situation, it is
/// recommended you utilize the `name_prefix` attribute and enable the
/// `create_before_destroy`. This will allow this provider
/// to create a new, updated `aws.iam.ServerCertificate` resource and replace it in
/// dependant resources before attempting to destroy the old version.
///
/// ## Import
///
/// Using `pulumi import`, import IAM Server Certificates using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/serverCertificate:ServerCertificate certificate example.com-certificate-until-2018
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod server_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerCertificateArgs {
        /// The contents of the public key certificate in
        /// PEM-encoded format.
        #[builder(into)]
        pub certificate_body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The contents of the certificate chain.
        /// This is typically a concatenation of the PEM-encoded public key certificates
        /// of the chain.
        #[builder(into, default)]
        pub certificate_chain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Server Certificate. Do not include the
        /// path in this value. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM path for the server certificate.  If it is not
        /// included, it defaults to a slash (/). If this certificate is for use with
        /// AWS CloudFront, the path must be in format `/cloudfront/your_path_here`.
        /// See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more details on IAM Paths.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the private key in PEM-encoded format.
        #[builder(into)]
        pub private_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of resource tags for the server certificate. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** AWS performs behind-the-scenes modifications to some certificate files if they do not adhere to a specific format. These modifications will result in this provider forever believing that it needs to update the resources since the local and AWS file contents will not match after theses modifications occur. In order to prevent this from happening you must ensure that all your PEM-encoded files use UNIX line-breaks and that `certificate_body` contains only one certificate. All other certificates should go in `certificate_chain`. It is common for some Certificate Authorities to issue certificate files that have DOS line-breaks and that are actually multiple certificates concatenated together in order to form a full certificate chain.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerCertificateResult {
        /// The Amazon Resource Name (ARN) specifying the server certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The contents of the public key certificate in
        /// PEM-encoded format.
        pub certificate_body: pulumi_gestalt_rust::Output<String>,
        /// The contents of the certificate chain.
        /// This is typically a concatenation of the PEM-encoded public key certificates
        /// of the chain.
        pub certificate_chain: pulumi_gestalt_rust::Output<Option<String>>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) on which the certificate is set to expire.
        pub expiration: pulumi_gestalt_rust::Output<String>,
        /// The name of the Server Certificate. Do not include the
        /// path in this value. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The IAM path for the server certificate.  If it is not
        /// included, it defaults to a slash (/). If this certificate is for use with
        /// AWS CloudFront, the path must be in format `/cloudfront/your_path_here`.
        /// See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more details on IAM Paths.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The contents of the private key in PEM-encoded format.
        pub private_key: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags for the server certificate. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** AWS performs behind-the-scenes modifications to some certificate files if they do not adhere to a specific format. These modifications will result in this provider forever believing that it needs to update the resources since the local and AWS file contents will not match after theses modifications occur. In order to prevent this from happening you must ensure that all your PEM-encoded files use UNIX line-breaks and that `certificate_body` contains only one certificate. All other certificates should go in `certificate_chain`. It is common for some Certificate Authorities to issue certificate files that have DOS line-breaks and that are actually multiple certificates concatenated together in order to form a full certificate chain.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) when the server certificate was uploaded.
        pub upload_date: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerCertificateArgs,
    ) -> ServerCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_body_binding = args.certificate_body.get_output(context);
        let certificate_chain_binding = args.certificate_chain.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let path_binding = args.path.get_output(context);
        let private_key_binding = args.private_key.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/serverCertificate:ServerCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateBody".into(),
                    value: &certificate_body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateChain".into(),
                    value: &certificate_chain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerCertificateResult {
            arn: o.get_field("arn"),
            certificate_body: o.get_field("certificateBody"),
            certificate_chain: o.get_field("certificateChain"),
            expiration: o.get_field("expiration"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            path: o.get_field("path"),
            private_key: o.get_field("privateKey"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            upload_date: o.get_field("uploadDate"),
        }
    }
}
