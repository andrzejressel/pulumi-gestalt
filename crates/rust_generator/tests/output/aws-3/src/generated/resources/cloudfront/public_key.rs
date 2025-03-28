/// ## Example Usage
///
/// The following example below creates a CloudFront public key.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudfront:PublicKey
///     properties:
///       comment: test public key
///       encodedKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: public_key.pem
///           return: result
///       name: test_key
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Public Key using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/publicKey:PublicKey example K3D5EWEUDCCXON
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod public_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicKeyArgs {
        /// An optional comment about the public key.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The encoded public key that you want to add to CloudFront to use with features like field-level encryption.
        #[builder(into)]
        pub encoded_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for the public key. By default generated by this provider. Note: Do not set if using the key's id in another resource (e.g. KeyGroup) since it will result in a dependency error from AWS. Instead, it is recommended to use Pulumi autonaming by leaving this property unset (default behavior) or set the `namePrefix` property to allow the provider to autoname the resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the public key. Conflicts with `name`.
        ///
        /// **NOTE:** When setting `encoded_key` value, there needs a newline at the end of string. Otherwise, multiple runs of pulumi will want to recreate the `aws.cloudfront.PublicKey` resource.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PublicKeyResult {
        /// Internal value used by CloudFront to allow future updates to the public key configuration.
        pub caller_reference: pulumi_gestalt_rust::Output<String>,
        /// An optional comment about the public key.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The encoded public key that you want to add to CloudFront to use with features like field-level encryption.
        pub encoded_key: pulumi_gestalt_rust::Output<String>,
        /// The current version of the public key. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name for the public key. By default generated by this provider. Note: Do not set if using the key's id in another resource (e.g. KeyGroup) since it will result in a dependency error from AWS. Instead, it is recommended to use Pulumi autonaming by leaving this property unset (default behavior) or set the `namePrefix` property to allow the provider to autoname the resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name for the public key. Conflicts with `name`.
        ///
        /// **NOTE:** When setting `encoded_key` value, there needs a newline at the end of string. Otherwise, multiple runs of pulumi will want to recreate the `aws.cloudfront.PublicKey` resource.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PublicKeyArgs,
    ) -> PublicKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let encoded_key_binding = args.encoded_key.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/publicKey:PublicKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encodedKey".into(),
                    value: &encoded_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PublicKeyResult {
            caller_reference: o.get_field("callerReference"),
            comment: o.get_field("comment"),
            encoded_key: o.get_field("encodedKey"),
            etag: o.get_field("etag"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
        }
    }
}
