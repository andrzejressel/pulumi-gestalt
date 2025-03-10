/// Provides a Lightsail Key Pair, for use with Lightsail Instances. These key pairs
/// are separate from EC2 Key Pairs, and must be created or imported for use with
/// Lightsail.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ### Create New Key Pair
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lgKeyPair = key_pair::create(
///         "lgKeyPair",
///         KeyPairArgs::builder().name("lg_key_pair").build_struct(),
///     );
/// }
/// ```
///
/// ### Create New Key Pair with PGP Encrypted Private Key
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lgKeyPair = key_pair::create(
///         "lgKeyPair",
///         KeyPairArgs::builder()
///             .name("lg_key_pair")
///             .pgp_key("keybase:keybaseusername")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Existing Public Key Import
///
/// ```yaml
/// resources:
///   lgKeyPair:
///     type: aws:lightsail:KeyPair
///     name: lg_key_pair
///     properties:
///       name: importing
///       publicKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ~/.ssh/id_rsa.pub
///           return: result
/// ```
///
/// ## Import
///
/// You cannot import Lightsail Key Pairs because the private and public key are only available on initial creation.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_pair {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyPairArgs {
        /// The name of the Lightsail Key Pair. If omitted, a unique name will be generated by this provider
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional PGP key to encrypt the resulting private key material. Only used when creating a new key pair
        #[builder(into, default)]
        pub pgp_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The public key material. This public key will be imported into Lightsail
        #[builder(into, default)]
        pub public_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the collection. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** a PGP key is not required, however it is strongly encouraged. Without a PGP key, the private key material will be stored in state unencrypted.`pgp_key` is ignored if `public_key` is supplied.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyPairResult {
        /// The ARN of the Lightsail key pair.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The MD5 public key fingerprint for the encrypted private key.
        pub encrypted_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// the private key material, base 64 encoded and encrypted with the given `pgp_key`. This is only populated when creating a new key and `pgp_key` is supplied.
        pub encrypted_private_key: pulumi_gestalt_rust::Output<String>,
        /// The MD5 public key fingerprint as specified in section 4 of RFC 4716.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lightsail Key Pair. If omitted, a unique name will be generated by this provider
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// An optional PGP key to encrypt the resulting private key material. Only used when creating a new key pair
        pub pgp_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// the private key, base64 encoded. This is only populated when creating a new key, and when no `pgp_key` is provided.
        pub private_key: pulumi_gestalt_rust::Output<String>,
        /// The public key material. This public key will be imported into Lightsail
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the collection. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** a PGP key is not required, however it is strongly encouraged. Without a PGP key, the private key material will be stored in state unencrypted.`pgp_key` is ignored if `public_key` is supplied.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyPairArgs,
    ) -> KeyPairResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let pgp_key_binding = args.pgp_key.get_output(context);
        let public_key_binding = args.public_key.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/keyPair:KeyPair".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pgpKey".into(),
                    value: &pgp_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKey".into(),
                    value: &public_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyPairResult {
            arn: o.get_field("arn"),
            encrypted_fingerprint: o.get_field("encryptedFingerprint"),
            encrypted_private_key: o.get_field("encryptedPrivateKey"),
            fingerprint: o.get_field("fingerprint"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            pgp_key: o.get_field("pgpKey"),
            private_key: o.get_field("privateKey"),
            public_key: o.get_field("publicKey"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
