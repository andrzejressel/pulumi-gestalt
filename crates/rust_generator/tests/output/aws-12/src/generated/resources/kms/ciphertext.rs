/// The KMS ciphertext resource allows you to encrypt plaintext into ciphertext
/// by using an AWS KMS customer master key. The value returned by this resource
/// is stable across every apply. For a changing ciphertext value each apply, see
/// the `aws.kms.Ciphertext` data source.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let oauth = ciphertext::create(
///         "oauth",
///         CiphertextArgs::builder()
///             .key_id("${oauthConfig.keyId}")
///             .plaintext(
///                 "{\n  \"client_id\": \"e587dbae22222f55da22\",\n  \"client_secret\": \"8289575d00000ace55e1815ec13673955721b8a5\"\n}",
///             )
///             .build_struct(),
///     );
///     let oauthConfig = key::create(
///         "oauthConfig",
///         KeyArgs::builder().description("oauth config").is_enabled(true).build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod ciphertext {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CiphertextArgs {
        /// An optional mapping that makes up the encryption context.
        #[builder(into, default)]
        pub context: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Globally unique key ID for the customer master key.
        #[builder(into)]
        pub key_id: pulumi_gestalt_rust::Input<String>,
        /// Data to be encrypted. Note that this may show up in logs, and it will be stored in the state file.
        #[builder(into)]
        pub plaintext: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct CiphertextResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Base64 encoded ciphertext
        pub ciphertext_blob: pulumi_gestalt_rust::Output<String>,
        /// An optional mapping that makes up the encryption context.
        pub context: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Globally unique key ID for the customer master key.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// Data to be encrypted. Note that this may show up in logs, and it will be stored in the state file.
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CiphertextArgs,
    ) -> CiphertextResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CiphertextArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CiphertextResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CiphertextArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CiphertextResult {
        let context_binding = args.context.get_output(ctx);
        let key_id_binding = args.key_id.get_output(ctx);
        let plaintext_binding = args.plaintext.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kms/ciphertext:Ciphertext".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "context".into(),
                    value: &context_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plaintext".into(),
                    value: &plaintext_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CiphertextResult {
            id: o.get_id(),
            urn: o.get_urn(),
            ciphertext_blob: o.get_field("ciphertextBlob"),
            context: o.get_field("context"),
            key_id: o.get_field("keyId"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
