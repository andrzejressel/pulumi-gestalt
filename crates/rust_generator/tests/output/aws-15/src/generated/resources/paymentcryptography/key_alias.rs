/// Resource for managing an AWS Payment Cryptography Control Plane Key Alias.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:paymentcryptography:Key
///     properties:
///       exportable: true
///       keyAttributes:
///         - keyAlgorithm: TDES_3KEY
///           keyClass: SYMMETRIC_KEY
///           keyUsage: TR31_P0_PIN_ENCRYPTION_KEY
///           keyModesOfUse:
///             - decrypt: true
///               encrypt: true
///               wrap: true
///               unwrap: true
///   testKeyAlias:
///     type: aws:paymentcryptography:KeyAlias
///     name: test
///     properties:
///       aliasName: alias/test-alias
///       keyArn: ${test.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Payment Cryptography Control Plane Key Alias using the `alias/4681482429376900170`. For example:
///
/// ```sh
/// $ pulumi import aws:paymentcryptography/keyAlias:KeyAlias example alias/4681482429376900170
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod key_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyAliasArgs {
        /// Name of the Key Alias.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub alias_name: pulumi_gestalt_rust::Input<String>,
        /// ARN of the key.
        #[builder(into, default)]
        pub key_arn: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyAliasResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Name of the Key Alias.
        ///
        /// The following arguments are optional:
        pub alias_name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the key.
        pub key_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyAliasArgs,
    ) -> KeyAliasResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyAliasArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> KeyAliasResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyAliasArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> KeyAliasResult {
        let alias_name_binding = args.alias_name.get_output(ctx);
        let key_arn_binding = args.key_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:paymentcryptography/keyAlias:KeyAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aliasName".into(),
                    value: &alias_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyArn".into(),
                    value: &key_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        KeyAliasResult {
            id: o.get_id(),
            urn: o.get_urn(),
            alias_name: o.get_field("aliasName"),
            key_arn: o.get_field("keyArn"),
        }
    }
}
