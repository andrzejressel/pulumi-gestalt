/// An alias from a key/certificate pair.
///
/// To get more information about KeystoresAliasesKeyCertFile, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases)
/// * How-to Guides
///     * [Keystores Aliases](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases)
///
/// ## Import
///
/// KeystoresAliasesKeyCertFile can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}`
///
/// * `{{org_id}}/{{environment}}/{{keystore}}/{{alias}}`
///
/// When using the `pulumi import` command, KeystoresAliasesKeyCertFile can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile default organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile default {{org_id}}/{{environment}}/{{keystore}}/{{alias}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod keystores_aliases_key_cert_file {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeystoresAliasesKeyCertFileArgs {
        /// Alias Name
        #[builder(into)]
        pub alias: pulumi_gestalt_rust::Input<String>,
        /// Cert content
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cert: pulumi_gestalt_rust::Input<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        #[builder(into, default)]
        pub certs_info: pulumi_gestalt_rust::Input<
            Option<super::super::types::apigee::KeystoresAliasesKeyCertFileCertsInfo>,
        >,
        /// Environment associated with the alias
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::Input<String>,
        /// Private Key content, omit if uploading to truststore
        #[builder(into, default)]
        pub key: pulumi_gestalt_rust::Input<Option<String>>,
        /// Keystore Name
        #[builder(into)]
        pub keystore: pulumi_gestalt_rust::Input<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::Input<String>,
        /// Password for the Private Key if it's encrypted
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeystoresAliasesKeyCertFileResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Alias Name
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Cert content
        ///
        ///
        /// - - -
        pub cert: pulumi_gestalt_rust::Output<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        pub certs_info: pulumi_gestalt_rust::Output<
            super::super::types::apigee::KeystoresAliasesKeyCertFileCertsInfo,
        >,
        /// Environment associated with the alias
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// Private Key content, omit if uploading to truststore
        pub key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Keystore Name
        pub keystore: pulumi_gestalt_rust::Output<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Password for the Private Key if it's encrypted
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional.Type of Alias
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeystoresAliasesKeyCertFileArgs,
    ) -> KeystoresAliasesKeyCertFileResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeystoresAliasesKeyCertFileArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> KeystoresAliasesKeyCertFileResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeystoresAliasesKeyCertFileArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> KeystoresAliasesKeyCertFileResult {
        let alias_binding = args.alias.get_output(ctx);
        let cert_binding = args.cert.get_output(ctx);
        let certs_info_binding = args.certs_info.get_output(ctx);
        let environment_binding = args.environment.get_output(ctx);
        let key_binding = args.key.get_output(ctx);
        let keystore_binding = args.keystore.get_output(ctx);
        let org_id_binding = args.org_id.get_output(ctx);
        let password_binding = args.password.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: &alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cert".into(),
                    value: &cert_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certsInfo".into(),
                    value: &certs_info_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: &environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keystore".into(),
                    value: &keystore_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        KeystoresAliasesKeyCertFileResult {
            id: o.get_id(),
            urn: o.get_urn(),
            alias: o.get_field("alias"),
            cert: o.get_field("cert"),
            certs_info: o.get_field("certsInfo"),
            environment: o.get_field("environment"),
            key: o.get_field("key"),
            keystore: o.get_field("keystore"),
            org_id: o.get_field("orgId"),
            password: o.get_field("password"),
            type_: o.get_field("type"),
        }
    }
}
