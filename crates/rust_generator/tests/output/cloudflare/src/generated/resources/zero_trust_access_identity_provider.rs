/// Provides a Cloudflare Access Identity Provider resource. Identity
/// Providers are used as an authentication or authorisation source
/// within Access.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let githubOauth = zero_trust_access_identity_provider::create(
///         "githubOauth",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     ZeroTrustAccessIdentityProviderConfig::builder().clientId("example")
///                     .clientSecret("secret_key").build_struct(),
///                 ],
///             )
///             .name("GitHub OAuth")
///             .type_("github")
///             .build_struct(),
///     );
///     let jumpcloudSaml = zero_trust_access_identity_provider::create(
///         "jumpcloudSaml",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     ZeroTrustAccessIdentityProviderConfig::builder()
///                     .attributes(vec!["email", "username",])
///                     .idpPublicCert("MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg\nuTnQyuOqPuHbnN83d/2l1NSYKCbHt24o")
///                     .issuerUrl("jumpcloud").signRequest(false)
///                     .ssoTargetUrl("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess")
///                     .build_struct(),
///                 ],
///             )
///             .name("JumpCloud SAML")
///             .type_("saml")
///             .build_struct(),
///     );
///     let okta = zero_trust_access_identity_provider::create(
///         "okta",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     ZeroTrustAccessIdentityProviderConfig::builder()
///                     .apiToken("okta_api_token").clientId("example")
///                     .clientSecret("secret_key").oktaAccount("https://example.com")
///                     .build_struct(),
///                 ],
///             )
///             .name("Okta")
///             .type_("okta")
///             .build_struct(),
///     );
///     let pinLogin = zero_trust_access_identity_provider::create(
///         "pinLogin",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("PIN login")
///             .type_("onetimepin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustAccessIdentityProvider:ZeroTrustAccessIdentityProvider example <account_id>/<identity_provider_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_access_identity_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessIdentityProviderArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
        #[builder(into, default)]
        pub configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessIdentityProviderConfig>>,
        >,
        /// Friendly name of the Access Identity Provider configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for SCIM settings for a given IDP.
        #[builder(into, default)]
        pub scim_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessIdentityProviderScimConfig>>,
        >,
        /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessIdentityProviderResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
        pub configs: pulumi_gestalt_rust::Output<
            Vec<super::types::ZeroTrustAccessIdentityProviderConfig>,
        >,
        /// Friendly name of the Access Identity Provider configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for SCIM settings for a given IDP.
        pub scim_configs: pulumi_gestalt_rust::Output<
            Vec<super::types::ZeroTrustAccessIdentityProviderScimConfig>,
        >,
        /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustAccessIdentityProviderArgs,
    ) -> ZeroTrustAccessIdentityProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let configs_binding = args.configs.get_output(context);
        let name_binding = args.name.get_output(context);
        let scim_configs_binding = args.scim_configs.get_output(context);
        let type__binding = args.type_.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessIdentityProvider:ZeroTrustAccessIdentityProvider"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configs".into(),
                    value: &configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scimConfigs".into(),
                    value: &scim_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustAccessIdentityProviderResult {
            account_id: o.get_field("accountId"),
            configs: o.get_field("configs"),
            name: o.get_field("name"),
            scim_configs: o.get_field("scimConfigs"),
            type_: o.get_field("type"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
