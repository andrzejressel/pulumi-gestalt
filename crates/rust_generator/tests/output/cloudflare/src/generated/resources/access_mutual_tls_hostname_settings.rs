/// Provides a Cloudflare Access Mutual TLS Certificate Settings resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_mutual_tls_hostname_settings::create(
///         "example",
///         AccessMutualTlsHostnameSettingsArgs::builder()
///             .settings(
///                 vec![
///                     AccessMutualTlsHostnameSettingsSetting::builder().chinaNetwork(false)
///                     .clientCertificateForwarding(true).hostname("example.com")
///                     .build_struct(),
///                 ],
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Account level mTLS hostname settings import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example account/<account_id>
/// ```
///
/// Zone level mTLS hostname settings import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example zone/<zone_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod access_mutual_tls_hostname_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessMutualTlsHostnameSettingsArgs {
        /// The account identifier to target for the resource.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::Input<
            Option<Vec<super::types::AccessMutualTlsHostnameSettingsSetting>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessMutualTlsHostnameSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::AccessMutualTlsHostnameSettingsSetting>>,
        >,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessMutualTlsHostnameSettingsArgs,
    ) -> AccessMutualTlsHostnameSettingsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessMutualTlsHostnameSettingsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AccessMutualTlsHostnameSettingsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessMutualTlsHostnameSettingsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AccessMutualTlsHostnameSettingsResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let settings_binding = args.settings.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: &settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AccessMutualTlsHostnameSettingsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            settings: o.get_field("settings"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
